use ccsds_ndm::error::{CcsdsNdmError, DiagnosticNotation};
use ccsds_ndm::messages::opm::Opm;
use ccsds_ndm::{convert_opm, convert_opm_to_file, GenerateOptions, Notation, ParseOptions};
use serde_json::json;
use std::io::{Read, Write};
use std::path::PathBuf;

const EXIT_INVALID: i32 = 2;
const EXIT_UNSUPPORTED: i32 = 3;
const EXIT_RESOURCE: i32 = 4;
const EXIT_IO: i32 = 5;
const EXIT_USAGE: i32 = 64;

#[derive(Default)]
struct Common {
    input: Option<String>,
    notation: Option<Notation>,
    json: bool,
    max_input_bytes: Option<usize>,
    max_xml_depth: Option<usize>,
}

struct Convert {
    common: Common,
    target: Option<Notation>,
    output: Option<String>,
    max_output_bytes: Option<usize>,
    target_version: Option<String>,
}

fn notation(value: &str) -> Result<Notation, String> {
    match value {
        "kvn" => Ok(Notation::Kvn),
        "xml" => Ok(Notation::Xml),
        _ => Err(format!("unsupported notation '{value}'; use kvn or xml")),
    }
}

fn number(flag: &str, value: Option<String>) -> Result<usize, String> {
    value
        .ok_or_else(|| format!("{flag} requires a value"))?
        .parse()
        .map_err(|_| format!("{flag} requires a non-negative integer"))
}

fn parse_common(args: &[String], allow_convert: bool) -> Result<(Common, Vec<String>), String> {
    let mut common = Common::default();
    let mut rest = Vec::new();
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--format" | "--from" => {
                index += 1;
                common.notation =
                    Some(notation(args.get(index).ok_or_else(|| {
                        "notation flag requires a value".to_owned()
                    })?)?);
            }
            "--json" => common.json = true,
            "--max-input-bytes" => {
                index += 1;
                common.max_input_bytes =
                    Some(number("--max-input-bytes", args.get(index).cloned())?);
            }
            "--max-xml-depth" => {
                index += 1;
                common.max_xml_depth = Some(number("--max-xml-depth", args.get(index).cloned())?);
            }
            value if value.starts_with('-') && value != "-" => {
                if allow_convert {
                    rest.push(value.to_owned());
                } else {
                    return Err(format!("unknown option '{value}'"));
                }
            }
            value => {
                if common.input.replace(value.to_owned()).is_some() {
                    return Err("only one input file may be supplied".into());
                }
            }
        }
        index += 1;
    }
    Ok((common, rest))
}

fn parse_convert(args: &[String]) -> Result<Convert, String> {
    let mut common_args = Vec::new();
    let mut target = None;
    let mut output = None;
    let mut max_output_bytes = None;
    let mut target_version = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--to" => {
                index += 1;
                target = Some(notation(
                    args.get(index)
                        .ok_or_else(|| "--to requires a value".to_owned())?,
                )?);
            }
            "--output" | "-o" => {
                index += 1;
                output = Some(
                    args.get(index)
                        .ok_or_else(|| "--output requires a value".to_owned())?
                        .to_owned(),
                );
            }
            "--max-output-bytes" => {
                index += 1;
                max_output_bytes = Some(number("--max-output-bytes", args.get(index).cloned())?);
            }
            "--target-version" => {
                index += 1;
                target_version = Some(
                    args.get(index)
                        .ok_or_else(|| "--target-version requires a value".to_owned())?
                        .to_owned(),
                );
            }
            value => common_args.push(value.to_owned()),
        }
        index += 1;
    }
    let (common, unknown) = parse_common(&common_args, true)?;
    if let Some(option) = unknown.first() {
        return Err(format!("unknown option '{option}'"));
    }
    Ok(Convert {
        common,
        target,
        output,
        max_output_bytes,
        target_version,
    })
}

fn read_input(path: Option<&str>, max_bytes: Option<usize>) -> Result<String, CcsdsNdmError> {
    let mut bytes = Vec::new();
    match (path, max_bytes) {
        (Some(path), None) if path != "-" => {
            return std::fs::read_to_string(path).map_err(Into::into)
        }
        (_, None) => {
            std::io::stdin().read_to_end(&mut bytes)?;
        }
        (Some(path), Some(limit)) if path != "-" => {
            std::fs::File::open(path)?
                .take(limit.saturating_add(1) as u64)
                .read_to_end(&mut bytes)?;
        }
        (_, Some(limit)) => {
            std::io::stdin()
                .take(limit.saturating_add(1) as u64)
                .read_to_end(&mut bytes)?;
        }
    }
    if let Some(limit) = max_bytes {
        if bytes.len() > limit {
            return Err(CcsdsNdmError::ResourceLimitExceeded {
                resource: "input_document",
                limit,
                actual: bytes.len(),
            });
        }
    }
    Ok(String::from_utf8(bytes)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?)
}

fn detected(input: &str, selected: Option<Notation>) -> Notation {
    selected.unwrap_or_else(|| {
        if input.trim_start().starts_with('<') {
            Notation::Xml
        } else {
            Notation::Kvn
        }
    })
}

fn parse_options(common: &Common) -> ParseOptions {
    let mut options = ParseOptions {
        max_input_bytes: common.max_input_bytes,
        ..ParseOptions::default()
    };
    if let Some(depth) = common.max_xml_depth {
        options.max_xml_depth = depth;
    }
    options
}

fn report(error: &CcsdsNdmError, as_json: bool) {
    if as_json {
        let diagnostic = error.diagnostic();
        let notation = diagnostic.as_ref().map(|value| match value.notation {
            DiagnosticNotation::Kvn => "kvn",
            DiagnosticNotation::Xml => "xml",
        });
        eprintln!(
            "{}",
            json!({
                "severity": "error",
                "operation": diagnostic.as_ref().map(|value| match value.operation {
                    ccsds_ndm::error::DiagnosticOperation::Parse => "parse",
                    ccsds_ndm::error::DiagnosticOperation::Generate => "generate",
                }),
                "notation": notation,
                "message_kind": diagnostic.as_ref().map(|_| "opm"),
                "source_edition": diagnostic.as_ref().and_then(|value| value.source_edition),
                "target_edition": diagnostic.as_ref().and_then(|value| value.target_edition),
                "code": error.code(),
                "field_path": error.field_path(),
                "line": diagnostic.as_ref().and_then(|value| value.source_location.map(|location| location.0)),
                "column": diagnostic.as_ref().and_then(|value| value.source_location.map(|location| location.1)),
                "byte_offset": diagnostic.as_ref().and_then(|value| value.byte_offset),
                "original_token": diagnostic.as_ref().and_then(|value| value.original_token),
                "expected": diagnostic.as_ref().and_then(|value| value.expected),
                "recovery": null,
                "message": error.to_string(),
            })
        );
    } else {
        eprintln!("{error}");
    }
}

fn exit_for(error: &CcsdsNdmError) -> i32 {
    match error.code() {
        Some(code) if code.starts_with("resource.") => EXIT_RESOURCE,
        Some("parse.unsupported_input_version" | "generation.unsupported_output_version") => {
            EXIT_UNSUPPORTED
        }
        _ if error.as_io_error().is_some() => EXIT_IO,
        _ => EXIT_INVALID,
    }
}

fn validate(common: Common) -> Result<(), (CcsdsNdmError, bool)> {
    let input = read_input(common.input.as_deref(), common.max_input_bytes)
        .map_err(|error| (error, common.json))?;
    let options = parse_options(&common);
    match detected(&input, common.notation) {
        Notation::Kvn => Opm::from_kvn_with_options(&input, &options),
        Notation::Xml => Opm::from_xml_with_options(&input, &options),
    }
    .map(|_| ())
    .map_err(|error| (error, common.json))
}

fn convert(command: Convert) -> Result<(), (CcsdsNdmError, bool)> {
    let input = read_input(
        command.common.input.as_deref(),
        command.common.max_input_bytes,
    )
    .map_err(|error| (error, command.common.json))?;
    let source = detected(&input, command.common.notation);
    let target = command.target.ok_or_else(|| {
        (
            CcsdsNdmError::UnsupportedMessage("convert requires --to kvn|xml".into()),
            command.common.json,
        )
    })?;
    let parse_options = parse_options(&command.common);
    let mut generate_options = match command.target_version.as_deref() {
        None | Some("source") => GenerateOptions::source(),
        Some("latest") => GenerateOptions::latest(),
        Some(version) => GenerateOptions::version(version),
    };
    generate_options.max_output_bytes = command.max_output_bytes;

    match command.output.as_deref() {
        Some(path) if path != "-" => convert_opm_to_file(
            &input,
            PathBuf::from(path),
            source,
            target,
            &parse_options,
            &generate_options,
        ),
        _ => convert_opm(&input, source, target, &parse_options, &generate_options).and_then(
            |output| {
                std::io::stdout().write_all(output.as_bytes())?;
                Ok(())
            },
        ),
    }
    .map_err(|error| (error, command.common.json))
}

fn usage() {
    eprintln!(
        "usage:\n  ccsds-ndm validate [--format kvn|xml] [--json] [limits] [FILE|-]\n  ccsds-ndm convert [--from kvn|xml] --to kvn|xml [-o FILE|-] [--target-version source|latest|3.0] [--json] [limits] [FILE|-]"
    );
}

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next();
    let args: Vec<_> = args.collect();
    let result = match command.as_deref() {
        Some("--help" | "-h") => {
            usage();
            return;
        }
        Some("--version" | "-V") => {
            println!("ccsds-ndm {}", env!("CARGO_PKG_VERSION"));
            return;
        }
        Some("validate") => parse_common(&args, false).and_then(|(common, _)| {
            validate(common).map_err(|(error, json)| {
                report(&error, json);
                std::process::exit(exit_for(&error));
            })
        }),
        Some("convert") => parse_convert(&args).and_then(|command| {
            convert(command).map_err(|(error, json)| {
                report(&error, json);
                std::process::exit(exit_for(&error));
            })
        }),
        _ => Err("expected 'validate' or 'convert'".to_owned()),
    };
    if let Err(message) = result {
        eprintln!("{message}");
        usage();
        std::process::exit(EXIT_USAGE);
    }
}

# Project information
project = "CCSDS NDM"
copyright = "2025, Jochim"
author = "Jochim"

# General configuration
extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.napoleon",
    "sphinx.ext.viewcode",
    "sphinx.ext.intersphinx",
    "sphinx_tabs.tabs",
]

templates_path = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store", ".venv", "README.md"]

# Theme configuration
html_theme = "shibuya"
html_theme_options = {
    "github_url": "https://github.com/JochimMaene/ccsds-ndm",
    "nav_links": [
        {
            "title": "Rust API",
            "url": "https://docs.rs/ccsds-ndm/latest/ccsds_ndm/",
            "external": True,
        },
    ],
}

# Intersphinx configuration
intersphinx_mapping = {
    "python": ("https://docs.python.org/3", None),
}

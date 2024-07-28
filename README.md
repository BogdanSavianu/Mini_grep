# Grep Mimic Command

## Overview

This project aims to replicate the behavior of the grep shell command. It searches for a specific string within a file and returns all lines that contain that string.

# Features

	•	Case-sensitive search by default.
	•	Case-insensitive search available through an environment variable.

# Usage

### Command Line Arguments

The program requires two command line arguments:

	1.	Search String: The string to search for within the file.
	2.	File Path: The path to the file where the search will be performed.

### Example
cargo run -- "search_string" file.txt

## Case Insensitive Search

To perform a case-insensitive search, set the IGNORE_CASE environment variable to 1. This will make the search case insensitive for the lifetime of that terminal session.

### Example
IGNORE_CASE=1 cargo run -- "search_string" file.txt

#!/usr/bin/env pwsh
# LaunchPoc.ps1 - Script to list and run Rustica prototypes

# Define ANSI color codes for better display
$colors = @{
    title = "`e[1;36m"   # Bright Cyan (Bold)
    number = "`e[1;33m"  # Bright Yellow (Bold)
    name = "`e[0;37m"    # White
    description = "`e[0;36m" # Cyan
    reset = "`e[0m"      # Reset to default
}

# Function to get prototype descriptions (if available in README.md)
function Get-PrototypeDescriptions {
    $descriptionsMap = @{}
    
    # Try to read from the Prototypes/Readme.md file
    $readmePath = Join-Path $PSScriptRoot "Prototypes\Readme.md"
    if (Test-Path $readmePath) {
        $content = Get-Content $readmePath -Raw
        
        # Look for "## Current Prototypes" section and parse descriptions
        if ($content -match "## Current Prototypes(.*?)(?:##|$)") {
            $prototypesSection = $matches[1]
            $matches = [regex]::Matches($prototypesSection, '\d+\.\s\*\*(\d{3}-[\w-]+)\*\*:\s(.*)')
            
            foreach ($match in $matches) {
                $prototypeId = $match.Groups[1].Value
                $description = $match.Groups[2].Value.Trim()
                $descriptionsMap[$prototypeId] = $description
            }
        }
    }
    
    # For each prototype folder, also check for its own README.md
    $prototypesDir = Join-Path $PSScriptRoot "Prototypes"
    $prototypeFolders = Get-ChildItem -Path $prototypesDir -Directory | Where-Object { $_.Name -match '^\d{3}-' }
    
    foreach ($folder in $prototypeFolders) {
        $protoReadmePath = Join-Path $folder.FullName "README.md"
        if (Test-Path $protoReadmePath) {
            $content = Get-Content $protoReadmePath -Raw
            
            # Extract first paragraph as description if no description from main README
            if (-not $descriptionsMap.ContainsKey($folder.Name) -and $content -match '#.*?\r?\n\r?\n(.*?)(\r?\n\r?\n|$)') {
                $descriptionsMap[$folder.Name] = $matches[1].Trim()
            }
        }
    }
    
    return $descriptionsMap
}

# Get the path to the Prototypes directory
$prototypesPath = Join-Path $PSScriptRoot "Prototypes"

# Get all prototype directories
$prototypes = Get-ChildItem -Path $prototypesPath -Directory | 
    Where-Object { $_.Name -match '^\d{3}-' } | 
    Sort-Object Name

# Get descriptions for prototypes
$descriptions = Get-PrototypeDescriptions

# Clear the screen and display header
Clear-Host
Write-Host "$($colors.title)Rustica Prototype Launcher$($colors.reset)"
Write-Host "==============================="
Write-Host ""

# List available prototypes
Write-Host "Available prototypes:"
Write-Host ""

foreach ($i in 0..($prototypes.Count-1)) {
    $protoName = $prototypes[$i].Name
    $friendlyName = $protoName -replace '^\d{3}-', ''
    
    # Try to get description for this prototype
    $description = if ($descriptions.ContainsKey($protoName)) { 
        $descriptions[$protoName] 
    } else { 
        "No description available" 
    }
    
    # Display with number, name and description
    Write-Host "$($colors.number)$($i+1).$($colors.reset) $($colors.name)$friendlyName$($colors.reset)"
    Write-Host "   $($colors.description)$description$($colors.reset)"
}

Write-Host ""

# Ask user for selection
do {
    $selection = Read-Host "Enter the number of the prototype to run (1-$($prototypes.Count)), or 'q' to quit"
    
    if ($selection -eq 'q') {
        exit
    }
    
    $selectionInt = 0
    $validSelection = [int]::TryParse($selection, [ref]$selectionInt) -and 
                     $selectionInt -ge 1 -and 
                     $selectionInt -le $prototypes.Count
                     
    if (-not $validSelection) {
        Write-Host "Invalid selection. Please enter a number between 1 and $($prototypes.Count)."
    }
} while (-not $validSelection)

# Get the selected prototype
$selectedPrototype = $prototypes[$selectionInt - 1]
$selectedPrototypePath = $selectedPrototype.FullName

# Navigate to the prototype directory and run it
Write-Host ""
Write-Host "Running prototype: $($colors.name)$($selectedPrototype.Name)$($colors.reset)"
Write-Host "-------------------------------"
Write-Host ""

# Show a message about what we're doing
Write-Host "Changing to directory: $selectedPrototypePath"
Write-Host "Executing: cargo run"
Write-Host ""

# Actually change directory and run the prototype
Push-Location $selectedPrototypePath
try {
    cargo run
} 
finally {
    # Return to the original directory when done
    Pop-Location
}
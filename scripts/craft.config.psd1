# craft.config.psd1 - per-repo settings for start.ps1 and scripts\release.ps1.
# Standard: mcp-central-docs\standards\rust-artcraft\CRAFT_START_AND_RELEASE.md
@{
    Name        = 'puppycraft'
    Display     = 'PuppyCraft'
    AppPackage  = 'puppycraft-app'
    AppBin      = 'puppycraft-app'
    AppSmoke    = @()
    CliPackage  = 'puppycraft-mcp'
    CliBin      = 'puppycraft-mcp'
    CliSmoke    = @('status', '--json')
    McpArgs     = @('mcp')
    McpSmoke    = @('mcp')
    Installer   = $false
    Mcpb        = $false
}

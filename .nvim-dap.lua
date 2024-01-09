return {
  configurations = {
    rust = {
      {
        name = 'Debug lmml',
        type = 'codelldb',
        request = 'launch',
        program = function()
          return vim.fn.getcwd() .. './target/debug/lmml.exe'
        end,
        cwd = '${workspaceFolder}',
        stopOnEntry = false,
        args = { "load", "hoge" }
      }
    }
  }
}

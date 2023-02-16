local dap = require("dap")
dap.configurations.rust = {
	{
		name = "Launch",
		type = "codelldb",
		request = "launch",
		program = function()
			return vim.fn.getcwd() .. "/target/debug/catr"
		end,
		cwd = "${workspaceFolder}",
		stopOnEntry = false,
		args = {
			"tests/inputs/fox.txt",
		},
	},
	{
		name = "Launch No File",
		type = "codelldb",
		request = "launch",
		program = function()
			return vim.fn.getcwd() .. "/target/debug/catr"
		end,
		cwd = "${workspaceFolder}",
		stopOnEntry = false,
		args = {
			"abdeeeee",
		},
	},
}

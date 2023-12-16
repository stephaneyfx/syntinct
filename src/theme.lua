vim.opt.background = "dark"
vim.cmd("highlight clear")
for k, v in pairs(highlights) do
  vim.api.nvim_set_hl(0, k, v)
end

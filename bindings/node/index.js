try {
  module.exports = require("../../build/Release/tree_sitter_quarto_binding");
} catch (error1) {
  if (error1.code !== 'MODULE_NOT_FOUND') {
    throw error1;
  }
  try {
    module.exports = require("../../build/Debug/tree_sitter_quarto_binding");
  } catch (error2) {
    if (error2.code !== 'MODULE_NOT_FOUND') {
      throw error2;
    }
    throw error1
  }
}

try {
  module.exports.nodeTypeInfo = require("../../tree-sitter-quarto/src/node-types.json");
  module.exports.nodeTypeInfoInline = require("../../tree-sitter-quarto-inline/src/node-types.json");
} catch (_) {}


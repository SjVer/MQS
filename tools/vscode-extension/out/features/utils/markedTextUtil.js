"use strict";
exports.__esModule = true;
exports.textToMarkedString = void 0;
var vscode_1 = require("vscode");
function textToMarkedString(text) {
    return new vscode_1.MarkdownString(text.replace(/[\\`*_{}[\]()#+\-.!]/g, '\\$&'));
    // return text.replace(/[\\`*_{}[\]()#+\-.!]/g, '\\$&'); // escape markdown syntax tokens: http://daringfireball.net/projects/markdown/syntax#backslash
}
exports.textToMarkedString = textToMarkedString;

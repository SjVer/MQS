"use strict";
exports.__esModule = true;
exports.solveQuestionCallback = exports.refreshCodeLensCallback = exports.codelensDisposable = void 0;
var vscode_1 = require("vscode");
var codeLensProvider_1 = require("./codeLensProvider");
var codeLensProvider = new codeLensProvider_1.MQSCodeLensProvider();
// refreshes codelens
var refreshCodeLensCallback = function () {
    if (exports.codelensDisposable)
        exports.codelensDisposable.dispose();
    exports.codelensDisposable = vscode_1.languages.registerCodeLensProvider({ language: "mqs", scheme: "file" }, codeLensProvider);
};
exports.refreshCodeLensCallback = refreshCodeLensCallback;
// solves a question
var solveQuestionCallback = function (uri, name) {
    vscode_1.window.showInformationMessage("solved ".concat(name, " (").concat(uri, ")"));
    (0, exports.refreshCodeLensCallback)();
};
exports.solveQuestionCallback = solveQuestionCallback;

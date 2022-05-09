"use strict";
exports.__esModule = true;
exports.renameQuestionCallback = exports.setMQSResultCallback = exports.refreshCodeLensCallback = exports.codelensDisposable = void 0;
var vscode_1 = require("vscode");
var resultDatabase_1 = require("./resultDatabase");
var codeLensProvider_1 = require("./codeLensProvider");
// refreshes codelens
var refreshCodeLensCallback = function () {
    if (exports.codelensDisposable)
        exports.codelensDisposable.dispose();
    exports.codelensDisposable = vscode_1.languages.registerCodeLensProvider({ language: "mqs", scheme: "file" }, new codeLensProvider_1.MQSCodeLensProvider());
};
exports.refreshCodeLensCallback = refreshCodeLensCallback;
// sets a result
var setMQSResultCallback = function (pair) {
    (0, resultDatabase_1.setMQSResult)(pair);
    (0, exports.refreshCodeLensCallback)();
};
exports.setMQSResultCallback = setMQSResultCallback;
// renames a question
var renameQuestionCallback = function (editor, name, range) {
    var re = /\s*\?\s*[a-zA-Z_][a-zA-Z_0-9]*/gm;
    var offset = editor.document.getText(range).match(re)[0].length;
    // just append '1' to the name
    editor.edit(function (builder) {
        builder.insert(range.start.translate(0, offset), '1');
    });
};
exports.renameQuestionCallback = renameQuestionCallback;

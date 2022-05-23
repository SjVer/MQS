"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
exports.__esModule = true;
exports.getDocAsMarkdown = exports.getDocumentation = exports.callMQSLint = exports.MQSLintType = void 0;
var vscode_1 = require("vscode");
var backwardIterator_1 = require("./backwardIterator");
var child_process_1 = require("child_process");
var fs_1 = require("fs");
var path_1 = require("path");
var os_1 = require("os");
var async_1 = require("./async");
var MQSLintType;
(function (MQSLintType) {
    MQSLintType["Diagnostics"] = "diag";
    MQSLintType["Declaration"] = "decl";
    MQSLintType["Symbols"] = "symb";
})(MQSLintType = exports.MQSLintType || (exports.MQSLintType = {}));
;
;
;
;
var do_log = vscode_1.workspace.getConfiguration('mqs').get('logDebugInfo');
function log(message) {
    var optionalParams = [];
    for (var _i = 1; _i < arguments.length; _i++) {
        optionalParams[_i - 1] = arguments[_i];
    }
    if (do_log)
        console.log.apply(console, __spreadArray([message], optionalParams, false));
}
var blocked = false;
function parseLocation(loc, len) {
    var _a, _b;
    var parts = loc.split(':', 3);
    return {
        file: parts[0],
        line: (_a = Number.parseInt(parts[1]) - 1) !== null && _a !== void 0 ? _a : 0,
        column: (_b = Number.parseInt(parts[2]) - 1) !== null && _b !== void 0 ? _b : 0,
        length: len !== null && len !== void 0 ? len : 0
    };
}
function callMQSLint(document, type, position) {
    return __awaiter(this, void 0, void 0, function () {
        var tmpfile, mqsExec, args, output, data, result, result_1;
        return __generator(this, function (_a) {
            if (blocked)
                (0, async_1.waitUntil)(function () { return !blocked; }, 3000);
            blocked = true;
            log("\n\n\n====== LINT: ".concat(type.toUpperCase(), " ======"));
            tmpfile = (0, path_1.join)((0, os_1.tmpdir)(), (0, path_1.basename)(document.fileName) + '.mqslint_tmp');
            (0, fs_1.copyFileSync)(document.fileName, tmpfile);
            (0, fs_1.writeFileSync)(tmpfile, document.getText());
            mqsExec = vscode_1.workspace.getConfiguration('mqs').get('mqsExecutablePath');
            args = [tmpfile, "--lint=\"".concat(type, "\"")];
            // workspace.getConfiguration('mqs').get<string[]>('includeSearchPaths').forEach(path =>
            // 	cmd += ` --include=${path}`.replace('${workspaceFolder}', workspacefolder));
            log("lint cmd: ".concat(mqsExec, " ").concat(args.join(' ')));
            try {
                output = (0, child_process_1.spawnSync)(mqsExec, args, { encoding: 'utf-8', shell: true }).stdout;
            }
            catch (error) {
                console.warn("Failed to execute MQS binary: ".concat(error));
                blocked = false;
                return [2 /*return*/, Promise.reject()];
            }
            if (tmpfile !== document.fileName)
                (0, fs_1.rmSync)(tmpfile);
            blocked = false;
            // remove ansii escape codes
            output = output.replaceAll(RegExp(String.fromCharCode(0x1b) + "\\[([0-9]+;)?[0-9]+m", "g"), '');
            if (tmpfile !== document.fileName)
                output = output.replaceAll(tmpfile, document.fileName);
            log("lint output: " + output);
            try {
                data = JSON.parse(output);
            }
            catch (error) {
                console.warn("Failed to parse data returned by MQS binary:\n\"".concat(error, "\"\nData: \"").concat(output, "\""));
                return [2 /*return*/, Promise.reject()];
            }
            // change up some data if needed
            if (data['file'] === tmpfile)
                data['file'] == document.fileName;
            try {
                switch (type) {
                    case MQSLintType.Declaration: {
                        if (data['invalid'])
                            return [2 /*return*/, undefined];
                        result = {
                            position: parseLocation(data['location'], data['length'])
                        };
                        log(result);
                        return [2 /*return*/, Promise.resolve(result)];
                    }
                    case MQSLintType.Diagnostics: {
                        result_1 = { diagnostics: [] };
                        data.forEach(function (error) {
                            if (error['invalid'])
                                return;
                            // gather related information
                            var related = [];
                            error['related'].forEach(function (info) {
                                related.push({
                                    position: parseLocation(info['location'], info['length']),
                                    message: info['message']
                                });
                            });
                            // create error itself
                            result_1.diagnostics.push({
                                position: parseLocation(error['location'], error['length']),
                                message: error['message'],
                                code: error['code'],
                                type: error['severity'],
                                related: related
                            });
                        });
                        log(result_1);
                        return [2 /*return*/, Promise.resolve(result_1)];
                    }
                }
            }
            catch (e) {
                log(e);
            }
            return [2 /*return*/];
        });
    });
}
exports.callMQSLint = callMQSLint;
;
function getDocumentation(document, position) {
    return __awaiter(this, void 0, void 0, function () {
        var declaration, delcdoc, it, doc, c, params, paramRegex, match, ret, retRegex, match;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, callMQSLint(document, MQSLintType.Declaration, position)];
                case 1:
                    declaration = _a.sent();
                    if (!declaration)
                        return [2 /*return*/, Promise.reject("Symbol declaration not found.")];
                    return [4 /*yield*/, vscode_1.workspace.openTextDocument(vscode_1.Uri.file(declaration.position.file))];
                case 2:
                    delcdoc = _a.sent();
                    if (!delcdoc)
                        return [2 /*return*/, Promise.reject("Could not open file " + declaration.position.file)];
                    it = new backwardIterator_1.BackwardIterator(delcdoc, declaration.position.column, declaration.position.line);
                    // get newline before declaration
                    while (it.hasNext()) {
                        if (it.next() == "\n")
                            break;
                    }
                    ;
                    doc = "";
                    while (it.hasNext()) {
                        c = it.next();
                        if (c == "\n" && !doc.startsWith("--?")) {
                            // end of documentation, remove last (non-doc) line
                            doc = doc.slice(doc.indexOf("--?"));
                            // test if there's actually a documentation
                            if (!doc.startsWith('--?'))
                                doc = "";
                            break;
                        }
                        doc = c + doc;
                    }
                    // replace comment tokens with just newlines
                    doc = doc.replace(/\n?\-\-\?\s*/g, "\n") + "\n";
                    while (doc.startsWith("\n"))
                        doc = doc.slice(1);
                    params = [];
                    paramRegex = /\n\s*\@param\s+([0-9]+)\s+(.*)\n/;
                    while (paramRegex.test(doc)) {
                        match = doc.match(paramRegex);
                        doc = doc.replace(match[0], "\n");
                        params.push(match[2]);
                    }
                    ret = undefined;
                    retRegex = /\n\s*\@return\s+(.*)\n/;
                    if (retRegex.test(doc)) {
                        match = doc.match(retRegex);
                        doc = doc.replace(match[0], "\n");
                        // doc += `\n*@return* - ${match[1]}`;
                        ret = match[1];
                    }
                    return [2 /*return*/, Promise.resolve({ main: doc, params: params, ret: ret })];
            }
        });
    });
}
exports.getDocumentation = getDocumentation;
function getDocAsMarkdown(document, position) {
    return __awaiter(this, void 0, void 0, function () {
        var doc, text, i;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, getDocumentation(document, position)];
                case 1:
                    doc = _a.sent();
                    text = doc.main;
                    for (i = 0; i < doc.params.length; i++)
                        text += "\n*@param* `".concat(i, "` - ").concat(doc.params[i]);
                    if (doc.ret)
                        text += "\n*@return* - ".concat(doc.ret);
                    return [2 /*return*/, new vscode_1.MarkdownString(text.trim().replaceAll("\n", " \\\n"))];
            }
        });
    });
}
exports.getDocAsMarkdown = getDocAsMarkdown;

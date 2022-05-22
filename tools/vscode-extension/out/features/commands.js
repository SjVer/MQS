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
exports.__esModule = true;
exports.reviewQuestionCallback = exports.solveQuestionCallback = exports.refreshCodeLensCallback = exports.quickInfo = exports.QuickInfoMode = exports.codelensDisposable = void 0;
var child_process_1 = require("child_process");
var vscode_1 = require("vscode");
var codeLensProvider_1 = require("./codeLensProvider");
var MarkDownIt = require("markdown-it");
var codeLensProvider = new codeLensProvider_1.MQSCodeLensProvider();
var mqsQuickInfoExecutable = vscode_1.workspace.getConfiguration('mqs').get("mqsQuickInfoExecutablePath");
var mqsExecutable = vscode_1.workspace.getConfiguration('mqs').get("mqsExecutablePath");
var md = new MarkDownIt();
var QuickInfoMode;
(function (QuickInfoMode) {
    QuickInfoMode[QuickInfoMode["ExitCode"] = 0] = "ExitCode";
    QuickInfoMode[QuickInfoMode["Json"] = 1] = "Json";
})(QuickInfoMode = exports.QuickInfoMode || (exports.QuickInfoMode = {}));
function quickInfo(mode, command) {
    var _a, _b;
    var args = [];
    for (var _i = 2; _i < arguments.length; _i++) {
        args[_i - 2] = arguments[_i];
    }
    return __awaiter(this, void 0, void 0, function () {
        var cli_args, r;
        return __generator(this, function (_c) {
            try {
                cli_args = [command].concat(args);
                r = (0, child_process_1.spawnSync)(mqsQuickInfoExecutable, cli_args, { encoding: 'utf-8', shell: true });
                switch (mode) {
                    case QuickInfoMode.ExitCode: return [2 /*return*/, (_a = r.status) !== null && _a !== void 0 ? _a : 0];
                    case QuickInfoMode.Json: return [2 /*return*/, (_b = JSON.parse(r.stdout)) !== null && _b !== void 0 ? _b : {}];
                }
            }
            catch (e) {
                console.warn(e);
                vscode_1.window.showWarningMessage("Executing mqs-quickinfo failed. Please check if setting 'mqs.mqsQuickInfoExecutablePath' is valid.");
            }
            return [2 /*return*/];
        });
    });
}
exports.quickInfo = quickInfo;
// refreshes codelens
var refreshCodeLensCallback = function () {
    if (exports.codelensDisposable)
        exports.codelensDisposable.dispose();
    exports.codelensDisposable = vscode_1.languages.registerCodeLensProvider({ language: "mqs", scheme: "file" }, codeLensProvider);
};
exports.refreshCodeLensCallback = refreshCodeLensCallback;
// solves a question
var solveQuestionCallback = function (uri, name) {
    try {
        (0, child_process_1.spawnSync)(mqsExecutable, [uri.fsPath], { encoding: 'utf-8', shell: true });
    }
    catch (e) {
        console.warn(e);
        vscode_1.window.showWarningMessage("Executing mqs failed. Please check if setting 'mqs.mqsExecutablePath' is valid.");
    }
    (0, exports.refreshCodeLensCallback)();
};
exports.solveQuestionCallback = solveQuestionCallback;
// reviews a question
var reviewQuestionCallback = function (uri, name) {
    try {
        var args = [uri.fsPath, '--review', "--at=".concat(name)];
        var r = (0, child_process_1.spawnSync)(mqsExecutable, args, { encoding: 'utf-8', shell: true });
        if (r.error) {
            vscode_1.window.showErrorMessage("Failed to review '?".concat(name, "'. See interpreter output for more information."));
            return;
        }
        // window shit
        var panel = vscode_1.window.createWebviewPanel("mqsQuestionReview", "Review of `?".concat(name, "`"), { viewColumn: vscode_1.ViewColumn.Beside });
        var text = r.stdout.trim().replaceAll('\n', " \\\n").replaceAll("    ", "&emsp;&emsp;");
        panel.webview.html = md.render(text, process.env);
    }
    catch (e) {
        console.warn(e);
        vscode_1.window.showWarningMessage("Executing mqs failed. Please check if setting 'mqs.mqsExecutablePath' is valid.");
    }
    (0, exports.refreshCodeLensCallback)();
};
exports.reviewQuestionCallback = reviewQuestionCallback;

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
exports.MQSCodeLensProvider = void 0;
var commands_1 = require("./commands");
var questionRegex = /(?:^|\s*)\?\s*([a-zA-Z_][a-zA-Z0-9_]*)?/gm;
var commentStartRegex = /--\*(?!.*\*--)/gm;
var commentEndRegex = /\*--/gm;
function createMQSSolveCommand(uri, name) {
    return {
        title: "Solve question",
        tooltip: "Solve question \"?".concat(name, "\""),
        command: 'mqs.solveQuestion',
        arguments: [uri, name]
    };
}
function createMQSReviewCommand(uri, name, correct) {
    return {
        // title: `Review result (${correct ? "correct" : "incorrect"})`,
        title: "Review result (".concat(correct, ")"),
        tooltip: "Review the result of question \"?".concat(name, "\""),
        command: "mqs.reviewQuestion",
        arguments: [uri, name]
    };
}
var MQSCodeLensProvider = /** @class */ (function () {
    function MQSCodeLensProvider() {
    }
    MQSCodeLensProvider.prototype.provideCodeLenses = function (document) {
        return __awaiter(this, void 0, void 0, function () {
            var lenses, questions, _a, _b, _i, name, line, is_correct;
            return __generator(this, function (_c) {
                switch (_c.label) {
                    case 0:
                        lenses = [];
                        return [4 /*yield*/, (0, commands_1.quickInfo)(commands_1.QuickInfoMode.Json, "get-questions", document.uri.fsPath)];
                    case 1:
                        questions = _c.sent();
                        _a = [];
                        for (_b in questions)
                            _a.push(_b);
                        _i = 0;
                        _c.label = 2;
                    case 2:
                        if (!(_i < _a.length)) return [3 /*break*/, 6];
                        name = _a[_i];
                        line = document.lineAt(questions[name] - 1);
                        // push solve codelens
                        lenses.push({
                            isResolved: true,
                            range: line.range,
                            command: createMQSSolveCommand(document.uri, name)
                        });
                        return [4 /*yield*/, (0, commands_1.quickInfo)(commands_1.QuickInfoMode.ExitCode, "can-review-question", document.uri.fsPath, name)];
                    case 3:
                        if (!((_c.sent()) === 0)) return [3 /*break*/, 5];
                        return [4 /*yield*/, (0, commands_1.quickInfo)(commands_1.QuickInfoMode.ExitCode, "question-is-true", document.uri.fsPath, name)];
                    case 4:
                        is_correct = (_c.sent()) === 0;
                        lenses.push({
                            isResolved: true,
                            range: line.range,
                            command: createMQSReviewCommand(document.uri, name, is_correct)
                        });
                        _c.label = 5;
                    case 5:
                        _i++;
                        return [3 /*break*/, 2];
                    case 6: return [2 /*return*/, lenses];
                }
            });
        });
    };
    return MQSCodeLensProvider;
}());
exports.MQSCodeLensProvider = MQSCodeLensProvider;

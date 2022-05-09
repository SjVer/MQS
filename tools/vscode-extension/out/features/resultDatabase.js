"use strict";
exports.__esModule = true;
exports.getMQSResult = exports.setMQSResult = void 0;
;
var MQSResultDatabase = [];
function setMQSResult(result) {
    for (var _i = 0, MQSResultDatabase_1 = MQSResultDatabase; _i < MQSResultDatabase_1.length; _i++) {
        var pair = MQSResultDatabase_1[_i];
        if (pair.key.name != result.key.name
            || pair.key.uri != result.key.uri)
            continue;
        pair.value = result.value;
        return;
    }
    MQSResultDatabase.push(result);
}
exports.setMQSResult = setMQSResult;
function getMQSResult(key) {
    var result = null;
    for (var _i = 0, MQSResultDatabase_2 = MQSResultDatabase; _i < MQSResultDatabase_2.length; _i++) {
        var pair = MQSResultDatabase_2[_i];
        if (pair.key.name != key.name
            || pair.key.uri != key.uri)
            continue;
        result = pair.value;
        break;
    }
    return result;
}
exports.getMQSResult = getMQSResult;

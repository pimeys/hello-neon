"use strict";

const { promisify } = require("util");
const { engineNew, engineQuery } = require("./index.node");
const engineQueryAsync = promisify(engineQuery);

class QueryEngine {
    constructor(datamodel) {
        this.db = engineNew(datamodel);
    }

    query() {
        return engineQueryAsync.call(this.db);
    }
}

const qe = new QueryEngine("model A {}");

async function main() {
    for (var i = 0; i < 500000; i++) {
        await qe.query();
    }
}

main();

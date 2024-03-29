interface IStatusContract {
    services: object;
}

interface IServiceStatus {
    id: String;
    name: String;
    version: String;
    compiledAt: String,
    url: String,
    lastOk: number,
    lastError: String,
    envInfo: String,
    lastPingDuration: String,
    started: number
}

const floatingRegex = '[+\\-]?(?:0|[1-9]\\d*)(?:\\.\\d*)?(?:[eE][+\\-]?\\d+)?'

const envRegex = {
    'Time': new RegExp('Time:\\s*(.*)\\n'),
    'CPU': new RegExp('CPU:\\s*(.*)\\n'),
    'CPU Cache': new RegExp('CPU Cache:\\s*(.*)\\n'),
    'Kernel': new RegExp('Kernel:\\s*(.*)\\n'),
}

const benchmarkRegex = {
    'Load time': new RegExp(`Load time:\\s(${floatingRegex})`),
    'Run time': new RegExp(`Run time:\\s(${floatingRegex})`),
    'Throughput': new RegExp(`Throughput:\\s(${floatingRegex})`),
}

const samplingRegex = new RegExp('Samples:\\n([\\s\\d+\\n?]*)');
const latencyRegex = new RegExp('Latencies \\(.*\\):\\n([\\w\\d\\.%+\\:\\s\\d+\\n?]*)')

export function resultParser(rawData) {
    console.log(rawData);
    let result = {
        'basics': {},
        'benchmark_env': {},
    }
    let all_entries = Object.entries(envRegex);
    for (let i = 0; i < all_entries.length; i++) {
        const current_entry = all_entries[i];
        result['benchmark_env'][current_entry[0]] = current_entry[1].exec(rawData)[1];
    }

    all_entries = Object.entries(benchmarkRegex);
    for (let i = 0; i < all_entries.length; i++) {
        const current_entry = all_entries[i];
        result['basics'][current_entry[0]] = current_entry[1].exec(rawData)[1];
    }

    let samplingResult = samplingRegex.exec(rawData)[1];
    result['basics']['samplings'] = samplingResult.split('\n').map(item => item.trim()).filter(item => item);

    try {
        const latencyResult = latencyRegex.exec(rawData)[1];
        const parsed = latencyResult.split("\n").filter(item => item).map(item => item.trim().split(':'));
        console.log(parsed);
        const keys = parsed.map(item => item[0]);
        const values = parsed.map(item => parseInt(item[1].trim()));
        result["basics"]["latency"] = {
            labels: keys,
            values: values
        }
    } catch (error) {
        console.log("no latency found:", error);
    }
    return result;
}
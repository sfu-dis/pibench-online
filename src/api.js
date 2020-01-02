const API_ENDPOINT = "http://localhost:8000"

export async function fetchInstanceInfo(backendUrl) {
    const response = await fetch(`${backendUrl}/info`);
    const data = response.json();
    return data;
}

export async function postBenchmark(data) {
    const response = await fetch(`${API_ENDPOINT}/benchmark`, {
        method: 'post',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });
    return response.json();
}

export async function fetchServerStatus(pid) {
    const response = await fetch(`${API_ENDPOINT}/status?pid=${pid}`);
    const data = response.json();
    return data;
}

export async function removeWrapperApi(backendUrl, wrapperName) {
    const response = await fetch(`${backendUrl}/remove_wrapper`, {
        method: "post",
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ "wrapper_name": wrapperName })
    });
    const data = response.json();
    return data;
}


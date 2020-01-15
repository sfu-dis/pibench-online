// Copyright (c) Simon Fraser University. All rights reserved.
// Licensed under the MIT license.

// Authors:
// Xiangpeng Hao <xiangpeng_hao@sfu.ca>

export async function fetchInstanceInfo(backendUrl) {
    const response = await fetch(`${backendUrl}/info`);
    const data = response.json();
    return data;
}

export async function postBenchmark(backendUrl, data) {
    const response = await fetch(`${backendUrl}/benchmark`, {
        method: 'post',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });
    return response.json();
}

export async function fetchServerStatus(backendUrl, pid) {
    const response = await fetch(`${backendUrl}/status?pid=${pid}`);
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


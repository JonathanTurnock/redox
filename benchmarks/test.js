import http from 'k6/http';
import { check, sleep, fail } from 'k6';
import { Trend, Rate } from 'k6/metrics';
import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";

// Custom metrics
const responseTimes = new Trend('response_times');
const failedRequests = new Rate('failed_requests');

export function handleSummary(data) {
    return {
        "summary.html": htmlReport(data),
    };
}

export let options = {
    stages: [
        { duration: '15s', target: 500 }, // Ramp-up to 500 VUs
        { duration: '15s', target: 2000 }, // Ramp-up to 2000 VUs
    ],
    thresholds: {
        'failed_requests': ['rate<0.01'], // Allow <1% of requests to fail
        'response_times': ['p(95)<30'], // 95% of requests must complete below 15ms
    },
};

let maxVU = 0; // Track the maximum VUs used

export default function () {
    maxVU = Math.max(maxVU, __VU); // Update max VU dynamically

    const url = 'http://192.168.139.17:8080/';
    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    let res = http.get(url, params);

    // Record metrics
    responseTimes.add(res.timings.duration);
    failedRequests.add(res.status !== 200);

    // Check response status
    const success = check(res, {
        'is status 200': (r) => r.status === 200,
    });

    // Stop the test if a threshold breach occurs
    if (!success || failedRequests.rate > 0.01) {
        console.error(`Threshold breached at VU: ${__VU}, Iteration: ${__ITER}`);
        console.error(`Maximum VUs reached: ${maxVU}`);
        fail(`Test stopped due to threshold breach at VU: ${__VU}`);
    }
}

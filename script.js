import http from 'k6/http';

export const options = {
    vus: 128,
    duration: "1m",
};

export default function () {
    http.get('http://192.168.20.64:8081/hello');
}

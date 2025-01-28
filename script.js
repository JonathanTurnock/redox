import http from 'k6/http';

export const options = {
    vus: 24,
    duration: "5m",
};

export default function () {
    http.get('http://localhost:8080/');
}

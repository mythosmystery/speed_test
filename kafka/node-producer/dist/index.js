import { Kafka } from 'kafkajs';
const kafka = new Kafka({
    clientId: 'bonfire-server',
    brokers: ['10.13.37.32:9092']
});
const producer = kafka.producer();
await producer.connect();
const message = {
    region: {
        state: "CA",
        type: "State",
        value: ""
    },
    orgId: "cali_test",
};
await producer.send({
    topic: 'add-region',
    messages: [{ value: JSON.stringify(message) }]
});
console.log("Message sent", message);
//# sourceMappingURL=index.js.map
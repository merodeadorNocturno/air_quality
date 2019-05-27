const events = require('events');
const emitter = new events.EventEmitter();

const listener = () => console.log('execute listener');

emitter.addListener('connection', listener);

emitter.emit('connection');
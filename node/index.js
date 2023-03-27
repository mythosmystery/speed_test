const d = new Date()
const currentTime = d.getHours()
const currentOffset = d.getTimezoneOffset() / 60
let orgOffset = 8
const resultTime = orgOffset != undefined ? currentTime + currentOffset - orgOffset : currentTime
console.log(resultTime)

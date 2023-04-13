const sharedMemory = require('./index')


process.on("message", msg => {
  if (msg === "ready") {
    console.log('Read shared string in child process', sharedMemory.getString("string.link"))

    process.send("finish")
    process.exit()
  }
})
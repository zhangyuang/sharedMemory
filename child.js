const sharedMemory = require('./index')


process.on("message", msg => {
  if (msg === "ready") {
    const parentString = sharedMemory.getString("string.link")
    console.log('Read shared string in child process:', sharedMemory.getString("string.link"))
    const newString = parentString + "childstring"
    sharedMemory.setString("string.link", "chil")
    console.log('Modify shared string in child process', sharedMemory.getString("string.link"))

    process.send("finish")
    process.exit()
  }
})
const sharedMemory = require('./index')


function generateBigString() {
  let bigStr = '';
  for (let i = 0; i < 100; i++) {
    bigStr += 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. ';
  }
  return bigStr;
}
process.on("message", msg => {
  if (msg === "ready") {
    const parentString = sharedMemory.getString("string.link")
    console.log('Read shared string in child process:', sharedMemory.getString("string.link"))
    sharedMemory.setString("string.link", "chil")
    console.log('Modify shared string in child process', sharedMemory.getString("string.link"))

    process.send("finish")
    process.exit()
  }
})
// process.on("message", i => {

//   const big = generateBigString()
//   console.log(i, big)
//   sharedMemory.setString(String(i), big)
//   process.send(String(i))
// })

messages = document.getElementById("messages")
console.log("chat_window.js is running")
/*
send_btn.onclick = ((ev) => {
  console.log("message sent")

});

 */

let message_loop = new EventSource("./stream");
message_loop.onopen = (e) => {
  console.log("opened message loop")
}

message_loop.onmessage = (e) => {
  console.log(e)
  let data = JSON.parse(e.data)

    let name = data.name
    let text = data.text
    let from_current_user = data.from_current_user === "true"
    messages.innerHTML +=
      "<div class='" + (from_current_user ? " sent" : " received") + "'>" +
      "<div class='message'>" +
      "<div class='name'>" + name +
      "</div>" +
      "<div class='text'>" +
      text +
      "</div>" +
      "</div>" +
      "</div>"

  messages.scrollTo(0, messages.scrollHeight)
}

let new_message = document.getElementById("new_message_box")
let submit_btn = document.getElementById("send_message")
submit_btn.onclick = (e) => {

  console.log("submit was clicked")
  console.log(e)

  var result = new XMLHttpRequest();
  result.open("POST", "/send_message")
  data = []
  let my_data = '{' +
    '"user":"' + new_message.textContent + '",' +
    '"text":"hello world"' +
    '}'

  result.setRequestHeader('Content-Type', 'application/json' )
  result.send(my_data)

  };



function init() {


}

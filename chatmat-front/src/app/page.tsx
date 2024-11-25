'use client'
import { HelloRequest } from "../proto/helloworld_pb";
import { GreeterClient } from "../proto/helloworld_grpc_web_pb";
import { useEffect, useState } from "react";
export default function Home() {
  let request;
  // define state
  let [messages, setMessages] = useState([]);
  useEffect(() => {
    const greeter = new GreeterClient("http://localhost:8080", null, null);
    request = new HelloRequest();

    const stream = greeter.serverStreaming(request, {});
    stream.on('data', (response) => {
      console.log('Response:', response.getMessage());
          })

    greeter.sayHello(request, {}, (err, response) => {
      if (err) {
        console.error('Error:', err);
      } else {
        console.log('Response:', response.getMessage());
      }
    });
  });
  return (
    <div>
    {messages.map((msg) => 
        <div>{msg}</div>
              )
    }
    </div>
  );
}

'use client'
import { HelloRequest } from "../proto/helloworld_pb";
import { GreeterClient } from "../proto/helloworld_grpc_web_pb";
import { useEffect } from "react";
export default function Home() {
  let request;
  useEffect(() => {
    const greeter = new GreeterClient("http://localhost:8080", null, null);
    request = new HelloRequest();

    greeter.sayHello(request, {}, (err, response) => {
      if (err) {
        console.error('Error:', err);
      } else {
        console.log('Response:', response.getMessage());
      }
    });
  });
  return (
    <div></div>
  );
}

'use client'

import React, { FormEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";


export default function Home() { 
  const [url, setUrl] = useState<String>("");
  const [result, setResult] = useState<String>("");

  const shortenUrl = (event: FormEvent) => {
    event.preventDefault();

    if(url.length == 0) throw Error("Can't be empty");
    invoke<string>("shorten_url", {url: url})
    .then((res) => {
      setResult(res);
    })
    .catch((err) => { 
      console.log(err);
     } )
  };

  const resetForm = () => {
    setUrl("");
    setResult("");
  }
  
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24 bg-gray-950">
      <h1 className="text-5xl font-bold">
      💻 URL Shortener 💻
      </h1>
      <form className="my-[10vw] flex flex-col items-center justify-start gap-[1rem]" onSubmit={shortenUrl}>
        <label htmlFor="url" className="opacity-[0.5]">Enter a URL to shorten:</label>
        <input type="text" name="url" className="w-[40rem] mb-[1vh] px-[1.5rem] py-[0.75rem] bg-gray-800 outline-none rounded-lg" onChange={(val) => setUrl(val.target.value)}/>
        <div className="">
          <input type="submit" value="Shorten ⚡" className="w-auto h-auto mr-[1rem] px-[2rem] py-[0.75rem] bg-emerald-700 rounded-lg cursor-pointer"/>
          <input type="reset" value="Reset 🔄" className="w-auto h-auto px-[2rem] py-[0.75rem] bg-blue-700 rounded-lg cursor-pointer" onClick={resetForm}/>
        </div>
      </form>

      {
        result.length > 0 && 
        <>
          <p className="mb-[1rem] opacity-[0.5]">Result:</p>
          <p className="text-xl">{result}</p>
        </>
      }

      <footer className="fixed bottom-[5vh]">
        <p className="text-xs">
          Created By <b>Adrian H.</b>
        </p>
      </footer>
    </main>
  );
}

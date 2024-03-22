"use client";

import React, { FormEvent, useState } from "react";
import Image from "next/image";
import { invoke } from "@tauri-apps/api/tauri";
import { writeText } from "@tauri-apps/api/clipboard";

import CopyIcon from "@assets/copy.svg";
import CheckIcon from "@assets/check.svg";

interface ResponseObject {
  code: number;
  url: string;
}

export default function Home() {
  const [url, setUrl] = useState<string>("");
  const [result, setResult] = useState<string>("");
  const [copied, setCopied] = useState<boolean>(false);
  const [error, setError] = useState<boolean>(false);
  const [errorMessage, setErrorMessagge] = useState<string>("");

  const shortenUrl = (event: FormEvent) => {
    event.preventDefault();

    if (url.length == 0) throw Error("Can't be empty");
    invoke<string>("shorten_url", { url: url })
      .then((res) => {
        let response_object: ResponseObject = JSON.parse(res);

        if (response_object.code == 0) {
          setError(true);
          setErrorMessagge("Failed to generate short URL");
          return;
        }

        setResult(response_object.url);
      })
      .catch((err) => {
        console.log(err);
      });
  };

  const copyUrl = async () => {
    await writeText(result);
    setCopied(true);
  };

  const resetForm = () => {
    setUrl("");
    setResult("");
  };

  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24 bg-gray-950 no-scrollbar">
      <h1 className="text-5xl font-bold">URL Shortener</h1>
      <form
        className="my-[10vw] flex flex-col items-center justify-start"
        onSubmit={shortenUrl}
      >
        <label htmlFor="url" className="mb-[1rem] opacity-[0.5]">
          Enter a URL to shorten:
        </label>
        <input
          type="text"
          name="url"
          className={`mb-[0.5rem] w-[40rem] px-[1.5rem] py-[0.75rem] bg-gray-800 outline-none rounded-lg ${
            error ? "border-[1px] border-red-400" : "border-none"
          }`}
          onChange={(val) => setUrl(val.target.value)}
          autoComplete="off"
        />
        {error && (
          <p className="pl-[0.5rem] w-full text-red-400 text-xs">
            {errorMessage}
          </p>
        )}
        <div className="mt-[4vh]">
          <input
            type="submit"
            value="Shorten ⚡"
            className="w-auto h-auto mr-[1rem] px-[2rem] py-[0.75rem] bg-emerald-700 rounded-lg cursor-pointer hover:bg-emerald-600 transition-all duration-250"
          />
          <input
            type="reset"
            value="Reset 🔄"
            className="w-auto h-auto px-[2rem] py-[0.75rem] bg-blue-700 rounded-lg cursor-pointer hover:bg-blue-600 transition-all duration-250"
            onClick={resetForm}
          />
        </div>
      </form>

      {result.length > 0 && !error && (
        <>
          <p className="mb-[0.5 rem] opacity-[0.5]">Result:</p>
          <div className="flex gap-[1rem]">
            <p className="text-xl">{result}</p>
            {!copied ? (
              <Image
                src={CopyIcon}
                alt="Copy Icon"
                className="w-[1rem] aspect-square opacity-[50%] hover:opacity-[75%] transition-all duration-250 cursor-pointer"
                onClick={copyUrl}
              />
            ) : (
              <Image
                src={CheckIcon}
                alt="Check Icon"
                className="w-[1rem] aspect-square opacity-[50%] hover:opacity-[75%] transition-all duration-250 cursor-pointer"
                onMouseLeave={() => setCopied(false)}
              />
            )}
          </div>
        </>
      )}

      <footer className="fixed bottom-[5vh] opacity-[75%]">
        <p className="text-xs">
          Created By <b>Adrian H.</b>
        </p>
      </footer>
    </main>
  );
}

"use client";

import React, { FormEvent, useState } from "react";
import Image from "next/image";
import { invoke } from "@tauri-apps/api/tauri";
import { writeText } from "@tauri-apps/api/clipboard";

import CopyIcon from "../assets/copy.svg";
import CheckIcon from "../assets/check.svg";

export default function Home() {
  const [url, setUrl] = useState<string>("");
  const [result, setResult] = useState<string>("");
  const [copied, setCopied] = useState<boolean>(false);

  const shortenUrl = (event: FormEvent) => {
    event.preventDefault();

    if (url.length == 0) throw Error("Can't be empty");
    invoke<string>("shorten_url", { url: url })
      .then((res) => {
        setResult(res);
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
        className="my-[10vw] flex flex-col items-center justify-start gap-[1rem]"
        onSubmit={shortenUrl}
      >
        <label htmlFor="url" className="opacity-[0.5]">
          Enter a URL to shorten:
        </label>
        <input
          type="text"
          name="url"
          className="w-[40rem] mb-[1vh] px-[1.5rem] py-[0.75rem] bg-gray-800 outline-none rounded-lg"
          onChange={(val) => setUrl(val.target.value)}
          autoComplete="off"
        />
        <div className="">
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

      {result.length > 0 && (
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

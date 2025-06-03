import { useMemo, useRef, useState, type FormEvent } from "react";
import axios, { AxiosError } from "axios";

import type { ServerData, ServerResponse } from "./types";
import { PlaylistDetails, Videos } from "./components";
import { toSeconds } from "./lib";

export function App() {
   const [data, setData] = useState<ServerData | null>(null);
   const [loading, setLoading] = useState(false);

   const searchRef = useRef<HTMLInputElement>(null);

   const totalWatchTime = useMemo(() => {
      return data?.videos.reduce((acc, item) => acc + toSeconds(item.duration), 0);
   }, [data]);

   async function checkPlaylist(event: FormEvent<HTMLFormElement>) {
      event.preventDefault();
      const value = searchRef.current?.value.trim();
      if (!value) return;

      let url: URL;
      try {
         url = new URL(value);
      } catch {
         window.alert("Enter a valid URL");
         return;
      }

      if (!url.hostname.includes("youtube.com")) {
         window.alert("Not a YouTube URL");
         return;
      }

      const listParam = url.searchParams.get("list");
      if (!listParam) {
         window.alert("Missing 'list' parameter");
         return;
      }

      setLoading(true);
      try {
         const response = await axios.post<ServerResponse>("http://localhost:8080/api/playlist-details", { list: listParam });
         setData(response.data.data);
      } catch (error) {
         let message = "An unexpected error was returned from the server";
         if (error instanceof AxiosError) message = error?.response?.data?.errors.join("\n");
         window.alert(message);
      } finally {
         setLoading(false);
      }
   }

   return (
      <main className="mx-auto min-h-screen w-full max-w-3xl px-5 py-10">
         <form onSubmit={checkPlaylist} className="flex items-center">
            <input
               type="text"
               className="border-border h-11 w-full rounded-l border bg-neutral-900 px-4 py-2 placeholder:text-sm"
               placeholder="Paste youtube playlist link here...."
               ref={searchRef}
            />
            <button
               type="submit"
               className="bg-primary hover:bg-primary/90 active:bg-primary/60 h-11 cursor-pointer rounded-r px-4 py-2 text-sm font-medium whitespace-nowrap shadow transition-colors"
            >
               Submit
            </button>
         </form>
         {data || loading ? (
            <PlaylistDetails
               channelName={data?.channelName}
               loading={loading}
               playlist={data?.playlist}
               totalWatchTime={totalWatchTime}
            />
         ) : null}
         {loading || !data ? null : <Videos videos={data?.videos} />}
      </main>
   );
}

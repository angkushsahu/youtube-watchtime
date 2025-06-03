import { useMemo } from "react";

import { toHumanReadableDate, toHumanReadableTime } from "../lib";
import type { ServerData } from "../types";

type Props = {
   loading: boolean;
   channelName?: string | null;
   playlist?: ServerData["playlist"] | null;
   totalWatchTime: number | undefined;
};

export function PlaylistDetails({ channelName, loading, playlist, totalWatchTime }: Props) {
   const sources = useMemo(
      () => [
         { media: "(min-width: 769px)", srcset: playlist?.thumbnails.maxres.url },
         { media: "(min-width: 641px)", srcset: playlist?.thumbnails.standard.url },
         { media: "(min-width: 481px)", srcset: playlist?.thumbnails.high.url },
         { media: "(min-width: 321px)", srcset: playlist?.thumbnails.medium.url },
         { media: "(max-width: 320px)", srcset: playlist?.thumbnails.default.url },
      ],
      [playlist]
   );

   return (
      <section className="my-10">
         <h1 className="mb-5 text-3xl font-medium">Playlist Details</h1>
         {loading || !channelName || !playlist ? (
            <>
               <div className="aspect-video w-full max-w-7xl animate-pulse rounded bg-neutral-800"></div>
               <div className="mt-4 h-8 w-11/12 animate-pulse rounded bg-neutral-800"></div>
               <div className="mt-2 mb-1 h-6 w-full max-w-1/2 animate-pulse rounded bg-neutral-800"></div>
               <div className="h-5 w-full max-w-1/4 animate-pulse rounded bg-neutral-800"></div>
            </>
         ) : (
            <>
               <picture>
                  {sources.map(({ media, srcset }) => (
                     <source key={media} media={srcset} srcSet={srcset} className="rounded" />
                  ))}
                  <img src={playlist.thumbnails.maxres.url} alt="Youtube thumbnail" className="h-auto w-full rounded" />
               </picture>
               <div className="mt-4">
                  <p className="text-2xl font-medium">{playlist.title}</p>
                  <p className="mt-2 mb-1 flex items-center gap-x-1.5 text-neutral-400">
                     <span className="font-medium text-white">{channelName}</span>
                     <span>•</span>
                     <span className="text-sm">{toHumanReadableDate(playlist.publishedAt)}</span>
                  </p>
                  <p className="flex items-center gap-x-1.5 text-sm text-neutral-400">
                     <span>
                        {playlist.totalVideos} {playlist.totalVideos === 1 ? "video" : "videos"}
                     </span>
                     <span className="text-base">•</span>
                     {totalWatchTime ? <span>{toHumanReadableTime(totalWatchTime)}</span> : <span>Loading ....</span>}
                  </p>
               </div>
            </>
         )}
      </section>
   );
}

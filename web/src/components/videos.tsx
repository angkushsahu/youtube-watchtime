import { useMemo } from "react";
import { toHumanReadableDate, toHumanReadableTime, toSeconds } from "../lib";
import type { ServerData } from "../types";

type Props = {
   videos?: ServerData["videos"] | null;
};

export function Videos({ videos }: Props) {
   if (!videos || videos.length < 1) {
      return <h1 className="mt-10 text-center text-xl text-neutral-400">There are no videos uploaded in this playlist</h1>;
   }

   return (
      <section>
         <h1 className="mb-5 text-3xl font-medium">Video Details</h1>
         {videos.map((video) => (
            <Video key={video.id} video={video} />
         ))}
      </section>
   );
}

function Video({ video }: { video: ServerData["videos"][number] }) {
   const sources = useMemo(
      () => [
         { media: "(min-width: 769px)", srcset: video?.thumbnails.maxres.url },
         { media: "(min-width: 641px)", srcset: video?.thumbnails.standard.url },
         { media: "(min-width: 481px)", srcset: video?.thumbnails.high.url },
         { media: "(min-width: 321px)", srcset: video?.thumbnails.medium.url },
         { media: "(max-width: 320px)", srcset: video?.thumbnails.default.url },
      ],
      [video]
   );

   return (
      <div className="my-4 flex gap-x-4">
         <picture>
            {sources.map(({ media, srcset }) => (
               <source key={media} media={srcset} srcSet={srcset} className="aspect-video w-full max-w-60 min-w-40 rounded" />
            ))}
            <img src={video.thumbnails.maxres.url} alt="A video" className="aspect-video w-full max-w-60 min-w-40 rounded" />
         </picture>
         <div className="grow">
            <p>{video.title}</p>
            <p className="my-1 text-sm text-neutral-400">{toHumanReadableDate(video.publishedAt)}</p>
            <p className="text-sm text-neutral-400">{toHumanReadableTime(toSeconds(video.duration))}</p>
         </div>
      </div>
   );
}

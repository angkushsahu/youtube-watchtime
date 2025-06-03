export function toSeconds(iso: string) {
   const regex = /PT(?:(\d+)H)?(?:(\d+)M)?(?:(\d+)S)?/;
   const match = iso.match(regex);
   if (!match) return 0;

   const [, h, m, s] = match.map((v) => Number(v ?? 0));
   return h * 3600 + m * 60 + s;
}

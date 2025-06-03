export function toHumanReadableTime(seconds: number) {
   const hours = Math.floor(seconds / 3600);
   const minutes = Math.floor((seconds % 3600) / 60);
   const secs = seconds % 60;

   const parts = [];
   if (hours) parts.push(`${hours} h`);
   if (minutes) parts.push(`${minutes} m`);
   if (secs || parts.length === 0) parts.push(`${secs} s`);

   return parts.join(" ");
}

export function toHumanReadableDate(dateString: string) {
   const date = new Date(dateString);
   return new Intl.DateTimeFormat("en-GB", {
      day: "2-digit",
      month: "short",
      year: "numeric",
   }).format(date);
}

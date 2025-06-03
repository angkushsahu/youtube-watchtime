type ThumbnailItem = {
   url: string;
   width: number;
   height: number;
};

type Thumbnail = {
   default: ThumbnailItem;
   medium: ThumbnailItem;
   high: ThumbnailItem;
   standard: ThumbnailItem;
   maxres: ThumbnailItem;
};

export type ServerData = {
   channelName: string;
   playlist: {
      publishedAt: string;
      title: string;
      thumbnails: Thumbnail;
      totalVideos: number;
   };
   videos: Array<{
      duration: string;
      publishedAt: string;
      position: number;
      id: string;
      thumbnails: Thumbnail;
      title: string;
   }>;
};

export type ServerResponse = {
   data: ServerData;
   message: string;
   success: boolean;
};

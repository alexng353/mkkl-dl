const fs = require("fs");
const fetch = require("node-fetch");

// fetch https://v6.mkklcdnv6tempv3.com/img/tab_6/00/09/26/oa952283/vol_1_chapter_1_to_you_2_000_years_from_now/1-o.jpg and save to file test.jpg
fetch(
  "https://v6.mkklcdnv6tempv3.com/img/tab_6/00/09/26/oa952283/vol_1_chapter_1_to_you_2_000_years_from_now/1-o.jpg"
).then((res) => {
  const dest = fs.createWriteStream("./test.jpg");
  res.body.pipe(dest);
});

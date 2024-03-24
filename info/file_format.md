# File Format:
## Requirements:
- Easy and fast to parse
- Supports
  - Bible Text
  - Highlights
  - Notes
  - Images
- Similar to existing formats

## Possible Implementations:
### Json:
```json
{
  "name": "bible_name",
  "version": "kjv",
  "notes": [ // any number of the Note Types
    // ...
  ],
  "images": [
    {
      "id": "some_identifier",
      "data": "...",
    },
    // ...
  ],
  "highlighters": [
    {
      "id": "highlighter_id",
      "color": "#FFFFFF", // Color hex
      "notes": "text"
    }
  ]
}
```

**Note Types:**
Note:
```json
{
  "type": "note",
  "location": "some:word:index", // Some book/chapter/verse location :shrug:
  "text": "some markdown text"
}
```
Cross-Ref:
```json
{
  "type": "cross-ref",
  "src": "some:word:index",
  "dst": "some:word:index",
  "text": "some markdown text"
}
```
Highlight:
```json
{
  "type": "highlight",
  "index": "Genesis:0:0:0-5", // word index of the bible
  "id": "highlight_id"
}
```
**Bible**
```json
{
  "version": "kjv",
  "description": "...",
  "copyright": "...",
  "books": [
    {
      "name": "Genesis",
      "testament": "old",
      "chapters": [
        {
          { // Chapter 1
            "verses": [
              { // Verse 1
                "text": "In the beginning...",
                "strongs": ["H0", "H1", /*...*/], // index, into each word also is optional
              },
              { // Verse 2
                "text": "And the earth",
                "strongs": ["H0", "H1", /*...*/],
              },
            ]
          },
          { // Chapter 2

          }
        }
      ]
    }
  ]
}
```
**Bible Index:**
```c#
"Genesis^10" // Genesis chapter 10
"Genesis^10-20" // 10th to 20th chapters

"Genesis^10:10" // Genesis 10 verse 10
"Genesis^10:10-20" // Genesis 10 verses 10 through 20
"Genesis^10:10-20:20" // Genesis verses 10:10 through 20:20

"Genesis^w10" // 10th word in Genesis
"Genesis^w10-50" // 10th to 50th word in genesis
```

**Notes**
- All markdown texts can have images embedded into them
- All basic text entries support markdown syntax
- You cannot have a word range between different books
- You can have a cross-ref between different books
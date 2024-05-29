# The-Book

## Practice

### Chapter 3
- [temperatures](temperatures/src/main.rs): Convert temperatures between Fahrenheit and Celsius.

- [fibonacci](fibonacci/src/main.rs): Generate the nth Fibonacci number.

- [lyrics](lyrics/src/main.rs): Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

### Chapter 8
- [statistics](statistics/src/main.rs): Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

- [pig_latin](pig_latin/src/main.rs): Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

- [company](company/src/main.rs): Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

### Chapter 17
- [blog](blog/src/main.rs)
    - Add a __reject__ method that changes the post’s state from [PendingReview](blog/src/lib.rs) back to [Draft](blog/src/lib.rs).
    - Require __two calls__ to [approve](blog/src/lib.rs) before the state can be changed to [Published](blog/src/lib.rs).
    - Allow users to add text content __only when a post is in the [Draft](blog/src/lib.rs) state__. Hint: have the state object responsible for what might change about the content but not responsible for modifying the [Post](blog/src/lib.rs).

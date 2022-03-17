# navigator
so i was getting annoyed trying to write menus for my console apps

i wanted a more consistent way to do menus with less effort

so i wrote my own menu framework. badly. with *macros.*

this is very cool because now rustfmt can't format the code inside the macros hooray

but hey, it'll probably do what i wanted it to do other than, uh, properly and methodically take user input

# todo
- actual homework
- d o c u m e n t a t i o n
- refactor the helpers
    - find all the instances of print_bar and see if they can be simplified
- get user input, not just user choices
    - insist on input, or cancel
- more customizable `nav`
    - loop on/off?
        - may be bad. forgetting to specify loop, or specifying wrong, would be harder to find than `nav` vs `pick`
    - 🌈*✨colors✨*🌈
    - `-- [ options header ] --`
    - `[option format]`
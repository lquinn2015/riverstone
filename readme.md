
# Example of Dithering Not floats allowed

The Repo here is a demonstration of doing Dithering but without floating point
numbers. Their are some accuracy issues but the code here is quick because there
are no floating point numbers! Most x86 core's have much faster integer units
i.e 8 execution units. Not to say this is highly optimized :) but I think its a
way of exercising a different part of a programmers toolset. It also relies on
you really understanding the algorithm being implemented because if you don't
you will round incorrectly and see random noise. 

It will also be a blog post repo to example some of the theory!


# Algos
 1) Flyod Steinberg
 1) Akinson
 1) Jarvis-Judice-Ninke
 1) Riemersma (unsupported) -- I would need to promote everything to i32 for
 reasonable perf which is doable but maybe a fun exercise

# Palettes 
 1) 1 bit color shown 
 1) GBA 
 1) Large palettle 30 colors alrightish
 1) purple palettle

# Input cat
![cat input](./test.png)

# Output
![1-bit output](./onebitcat.png)


# Dither floating point Forget about it! 

I want to take you on journay, recently I saw the game "Return of the Obra
Dinn[0]" And the art style was striking. It was aparently 1-bit colors but it
feel extremely vibrant. The styling artifact is aparently extremely retro and
was achieved through Dithering. There is a whole forum thread going over every
detail but I wanted to implement and explain a small subsection of it and give a
implementation that you could play with. And I want to do it without floaing
point numbers. It won't be perfect but it will be fun!

# Dithering

But what is Dithering? Dithering is a process by which we compress the color
spectrum of an image from the 256^3 RGB 8 color channels to something much less
say 1 bit, 256 color displays for terminals etc. This is because display
hardware from years begone could only have that many colors. LCD have incredible
depth and while that true color is super nice and can create very vibrant images
our eyes cannot actually perceive true color all that well. If you put 2
discrete RGB colors next to each other you would see a gradient of color not 2
discrete colors. If you put a matrix of pixel black and white next to each other 
the relative brightness density would make shades appear to your eyes. If you
inspect individual pixels you could tell black from white but typically you are
looking at an image not any one pixel. 

So Dithering can be done with an algorithm to reduce a huge color spectrum to a
smaller set while still perserving some of the shading effects but using the
brightness of neighboring pixels to give the perception of shading. There are a
lot of forms Bayer Matrix[1], Blue Noise [2], Halftone[3] which are good
parallel algorithm of order dithering but I am going to talk about Error
Propogation Algorithms like  Flyod Steinberg, Akinson, Jarvis-Judice-Ninke. But
let me explain the difference quickly! 

## Naive Dithering 
The Naive algorithm is to say for each pixel P in an Image if brightness(p) >
threshold use this color. This is extremely harsh an example shown below I
cheated a little bit in my rust program but the results should be very
identical. Either way the hard cut off looks bad 

```sh 
cargo run --release -- -i chibi.png -a 3 
```
![Chibi Harsh](./chibi_naive.png)

## Order Dithering 
In Order Dithering we have a Mask Matrix of many different threshold and apply
that as a filter. In algorithmic sense given a palettle P than 
for each pixel p in the image 
    for each color c in p 
        let c' = near_palette_color(c + r * M(x modn, y mod n) - 1/2). 
        set p.c = c'

Otherwise stated for each color we want to select that nearest color c' to after
adding in the threshold Matrix  $$M[x%n][y%n]$$ and normalizing by subtracting
1/2. The Normalize step is important because you increase the brightness if you
just add the threshold value. Normalization will become really important if you
don't use floating point numbers. I didn't program this method but included an
example from an online application using a Bayer matrix which is special matrix
used to correct colors in camera's since camera's cannot general see color but
only brightness. (they used floating point :( )

![Chibi Bayer](./chibi_bayer.png)

## Error Propogation
The Bayer matrix is nice but I wanted to do this with as little resources as
possible so i did Error Propogation with 16 bit accumlation in my program. Now
Error Propogation is 2 steps and it processes and image serially unlike Order
which can happen in any order. I name some kinds above but lets write the Floyd
Error Matrix 

$$ \frac{1/64} \begin{bmatrix}
  0 & * & 7

\end{bmatrix}
$$






Why does it work 

What is Dither formal

How do we do no floats



# Cititions:
[0] https://forums.tigsource.com/index.php?topic=40832.140

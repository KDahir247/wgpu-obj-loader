---
--- Generated by EmmyLua(https://github.com/EmmyLua)
--- Created by kdahi.
--- DateTime: 2022-04-09 9:29 a.m.
---


clip = audio_clip(".\\src\\audio\\Joyride.wav", true)


print(clip.samples)

print(clip)
local standard_clip = clip:standard();

local low_pass = standard_clip:low_pass(4);

print(low_pass)

print(low_pass.sample_rate)

--local source=  clip:play(1.0)


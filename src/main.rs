/*
CAB - Camera Auto Brightness
Date: 2024-02-13
Author: Huldar
*/

#![windows_subsystem = "windows"]  // In order to disable the console popup when the program is run

//use std::ops::Index;
use nokhwa::Camera;use nokhwa::utils;   //To access webcam
use brightness::Brightness; //To set system brightness
use futures::TryStreamExt;  //Async for brightness
use futures::executor::block_on;    //Async for brightness
//use std::fs::File; //For saving images to files to test
//use std::io::prelude::*; //For saving images to files to test

const RGB_MAX: u32 = 255;    //Used to scale environment brightness to screen brightness
const SCREEN_BRIGHTNESS_MAX: u32 = 100; //Used to scale environment brightness to screen brightness
const ENVIRONMENT_BRIGHTNESS_UNCERTAINTY: u32 = 30; //How uncertain we are about the environment brightness, used to scael environment brightness to screen brightness

fn main() {
    //println!("Starting CAB!");
    
    let user_brightness_preference:u32 = 35;    //How bright the user wants the screen (0-35)

    // Get camera brightness
    // first camera in system
    let camera_index = utils::CameraIndex::Index(0);
    // request the absolute highest resolution CameraFormat that can be decoded to RGB.
    let requested = utils::RequestedFormat::new::<nokhwa::pixel_format::RgbFormat>(nokhwa::utils::RequestedFormatType::AbsoluteHighestFrameRate);
    // make the camera
    let mut camera = match Camera::new(camera_index, requested){
        Ok(t) => {t},
        Err(e) => {println!("Camera creation error:\n{}", e);
                                return;},
    };

    // get a frame
    let frame = camera.frame().unwrap();
    //println!("Captured Single Frame of {}", frame.buffer().len());
    // decode into an ImageBuffer
    let decoded = frame.decode_image::<nokhwa::pixel_format::RgbFormat>().unwrap();
    let num_pixels = decoded.len();
    //println!("Decoded Frame of {}", num_pixels);
    
    // Save image to check
    //let mut file = File::create("test.jpg").unwrap();
    //file.write_all(frame.buffer());

    //Get environment brightness from image
    let mut environment_brightness:u32 = 0; //RGB values added together, max 255
    for (_decoded_index, decoded_rgb) in decoded.iter().enumerate() {   //_decoded_index not used, don't know how to remove it and keep same functionality, opportunity for improvement
        environment_brightness = environment_brightness + (*decoded_rgb as u32);
    }
    environment_brightness = environment_brightness/(num_pixels as u32) + ENVIRONMENT_BRIGHTNESS_UNCERTAINTY;
    //println!("Environment brightness: {}", environment_brightness);

    //Get screen brightness
    let brightness_display = get_brightness();
    match block_on(brightness_display){   //For future if we want to use the screen brightness outputted, add this in front: let screen_brightness = 
        Ok(t) => {t},
        Err(e) => {println!("Get brightness error:\n{}", e);
                        return;},
    };

    // Set screen brightness
    let new_screen_brightness = user_brightness_preference + SCREEN_BRIGHTNESS_MAX*environment_brightness/RGB_MAX;
    //println!("Environment brightness: {}", environment_brightness);
    //println!("New screen brightness: {}", new_screen_brightness);

    let set_brightness_display = set_brightness(new_screen_brightness);
    match block_on(set_brightness_display){
        Ok(t) => {t},
        Err(e) => {println!("Set brightness error:\n{}", e);
                                return;},
    };

    //Run finished
    //println!("CAB run finished!");
}

async fn get_brightness() -> Result<(), brightness::Error> {
    /*
    Should return average screen brightness (across all screens), is not returning that, need to figure out how to fix
    */
    //let mut screen_brightness:u32 = 0; //This variable is here for allowing cab to find the average brightness on multiple screens
    //let mut num_devices:u32 = 1;      //This variable is here for allowing cab to find the average brightness on multiple screens
    //println!("Reading brightness");
    //_dev is underscored while we haven't figured out how to return the screen_brightness value
    brightness::brightness_devices().try_for_each(|_dev: brightness::BrightnessDevice| async move {
        //num_devices = num_devices+1;
        //let name: String = dev.device_name().await?;
        //let value: u32 = dev.get().await?;
        //println!("Brightness of device {} is {}%", name, value);
        Ok(())
        //Err("Could not get brightness for device {}", name);
    }).await
}

async fn set_brightness(mut set_value: u32) -> Result<(), brightness::Error> {
    //Set to 100 (max brightness) if value is higher, note that u32 can not be smaler than 0
   if set_value > 100 {
        set_value = 100;
    }
    //println!("Setting brightness to {}", set_value);

    brightness::brightness_devices().try_for_each(|mut dev| async move {
        dev.set(set_value).await?;
        //let name = dev.device_name().await?;
        //let value = dev.get().await?;
        //println!("Brightness for device {} set to {}%", name, value);
        Ok(())
    }).await
}

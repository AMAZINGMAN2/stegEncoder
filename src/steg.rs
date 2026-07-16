use image::{self, RgbImage};
use std::mem::size_of;

// rust image documentation at https://docs.rs/image/latest/image/
// other useful examples can be found in the README of https://github.com/image-rs/image


const SIGNATURE : &[u8] = b"stegEncoder"; // a signature to make sure that the image being read will be encoded by this same encoder
const HEADERSIZE : usize = SIGNATURE.len() + size_of::<u32>();
pub fn tencryptimage(text: &str, imageData: &[u8]) -> Result<Vec<u8>, String>
{
    /*
     tencryptimage (TEXT ENCRYPT IMAGE) (string text, string imagePath, string outputPath)
     This function encrypts text into the lsb of an image using steganography
     */
    // opening the image and converting it to a rgb with 8 bit channels
    // let mut img = image::open(imagePath).map_err(|e| e.to_string())?.to_rgb8(); // this one is for local use
    let mut img = image::load_from_memory(imageData).map_err(|e| e.to_string())?.to_rgb8(); // this one is for use in nextjs website
    let imageSize = (img.height() as usize * img.width() as usize * 3) / 8;
    // checking if image is big enough to store the text
    let textSize = text.as_bytes().len() + HEADERSIZE;
    if imageSize < textSize
    {
        return Err(format!("Image size is {imageSize} and the text length is {textSize}. Try get a bigger image or use a smaller text").to_string());
    }
    // getting the length of the text and making it into bytes
    let length = text.as_bytes().len() as u32; // u32 so that it is the same for all operating systems
    let header = length.to_be_bytes();
    // creating a vector to be able to combine the header and the bytes of the text directly after
    let mut bytes = Vec::new();
    bytes.extend_from_slice(SIGNATURE); // Signature to make sure it is an image encoded by me
    bytes.extend_from_slice(&header); // stores the length of the encoded text
    bytes.extend_from_slice(text.as_bytes()); // the actual text
    let mut i = 0;
    for pixel in img.pixels_mut(){
        //0 is the r,g,b array of the pixel because its imported as a rgb8 right after opening
        let channels = pixel.0.iter_mut();
        if i >= bytes.len() * 8
        {break}
        for channel in channels{
            //checking if done writing the text
            if i >= bytes.len() * 8
            {break}
            // getting the ith byte then right shifting it by 7 - (i % 8) bits so that one bit is singled out
            // at a time meaning that it will right shift from 7 then after that 6 until 0 and wrap
            // around until all the bytes are done
            // i / 8 is the byte being read (note integer division ie. truncating decimals)
            // i % 8 is the bit being read
            let bit = (bytes[i / 8] >> (7 - (i % 8))) & 1;
            // writing the value of the bit to the least significant bit of the pixel
            *channel = (*channel & 0b11111110) | bit;
            i+=1;
        }
    }
    let mut output = Vec::new();

    img.write_to(
        &mut std::io::Cursor::new(&mut output),
        image::ImageFormat::Png
    )
        .map_err(|e| e.to_string())?;

    return Ok(output);
}



pub fn imagedecrypttext(imageData: &[u8]) -> Result<String, String>
{
    /*
     * imagedecrypttext (IMAGE DECRYPTION to TEXT) (String imagePath)
     * This function decrypts text out of an image;
     * essentially the inverse of tencryptimage()
     */
    // opens image
    // let img = image::open(imagePath).map_err(|e| e.to_string())?.to_rgb8(); // for local use
    let img = image::load_from_memory(imageData).map_err(|e| e.to_string())?.to_rgb8(); // for use in nextjs site 
    let mut bitPos = 0;
    let signature = readBytes(&img, SIGNATURE.len(), &mut bitPos)?;
    // check if the image has the signature
    if signature != SIGNATURE
    {
        return Err("Invalid image, make sure this image was encoded by this same steganography encoder".to_string());
    }
    let lengthBytes = readBytes(&img, size_of::<u32>(), &mut bitPos)?;
    let length = u32::from_be_bytes([lengthBytes[0],lengthBytes[1],lengthBytes[2],lengthBytes[3]]) as usize;
    let imageSize = (img.height() as usize * img.width() as usize * 3) / 8;
    if length + HEADERSIZE > imageSize
    {
        return Err("Invalid image, make sure this image was encoded by this same steganography encoder".to_string());
    }
    let text = readBytes(&img, length,&mut bitPos)?;
    return String::from_utf8(text).map_err(|e| e.to_string());
}



// a function that is made for the sake of abstraction as a result of using it multiple times in
// other functions, it serves as a function that reads an image starting from a specific bit for a
// specified amount of bits
fn readBytes(img : &RgbImage, amount : usize, startBit: &mut usize)-> Result<Vec<u8>, String>
{
    let mut bytes = Vec::new();
    let mut currentByte: u8 = 0;
    let mut nOfBits = 0;
    let channels = img.pixels().flat_map(|pixel| pixel.0.iter()).skip(*startBit);
    for channel in channels
    {
        // reading the current byte from the image bit by bit
        let bit = channel & 1;
        currentByte = (currentByte << 1) | bit;
        nOfBits += 1;

        // checking if the bit is the 8th bit of the iteration, then appending the current byte to the bytes vector
        if nOfBits == 8 {
            bytes.push(currentByte);
            currentByte = 0;
            nOfBits = 0;
            if bytes.len() == amount
            {
                //changing the value of the startBit variable to be the amounts Byte equivalent
                *startBit += amount * 8;
                return Ok(bytes);
            }
        }
    }
    return Err("Image does not contain enough data.".to_string());
}

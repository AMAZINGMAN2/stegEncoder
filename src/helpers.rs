use image::{self, RgbImage};
use std::mem::size_of;

const SIGNATURE : &[u8] = b"stegEncoder";
const HEADERSIZE : usize = SIGNATURE.len() + size_of::<u32>();
pub fn tencryptimage(text : &str, imagePath : &str, outputPath : &str)
{
    /*
     tencryptimage (TEXT ENCRYPT IMAGE) (string text, string imagePath, string outputPath)
     This function encrypts text into the lsb of an image using steganography
     */
    // opening the image and converting it to a rgb with 8 bit channels
    let mut img = image::open(imagePath).unwrap().to_rgb8();
    let imageSize = (img.height() as usize * img.width() as usize * 3) / 8;
    // checking if image is big enough to store the text
    if imageSize < (text.as_bytes().len() + HEADERSIZE)
    {
        panic!("IMAGE TOO SMALL");
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
    img.save(outputPath).unwrap();
}



pub fn imagedecrypttext(imagePath : &str) -> String
{
    /*
     * imagedecrypttext (IMAGE DECRYPTION to TEXT) (String imagePath)
     * This function decrypts text out of an image;
     * essentially the inverse of tencryptimage()
     */
    // opens image
    let img = image::open(imagePath).unwrap().to_rgb8();
    let mut bitPos = 0;
    let signature = readBytes(&img, SIGNATURE.len(), &mut bitPos);
    if signature != SIGNATURE
    {
        panic!("INVALID IMAGE");
    }
    let lengthBytes = readBytes(&img, size_of::<u32>(), &mut bitPos);
    let length = u32::from_be_bytes([lengthBytes[0],lengthBytes[1],lengthBytes[2],lengthBytes[3]]) as usize;
    let imageSize = (img.height() as usize * img.width() as usize * 3) / 8;
    if length + HEADERSIZE > imageSize.try_into().unwrap()
    {
        panic!("INVALID IMAGE");
    }
    let text = readBytes(&img, length,&mut bitPos);
    return String::from_utf8(text).unwrap();
}



fn readBytes(img : &RgbImage, amount : usize, startBit: &mut usize) -> Vec<u8>
{

    // let img = image::open(imagePath).unwrap().to_rgb8();
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
                *startBit += amount * 8;
                return bytes;
            }
        }
    }
    panic!("IMAGE DOES NOT CONTAIN ENOUGH DATA");
    }

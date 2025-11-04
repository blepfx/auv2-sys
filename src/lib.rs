#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

pub use core_foundation_sys as cf;

use cf::{
    base::{Boolean, OSStatus},
    dictionary::CFDictionaryRef,
    runloop::CFRunLoopRef,
    string::CFStringRef,
    url::CFURLRef,
};
use std::ffi::{c_char, c_void};

pub const noErr: OSStatus = 0;
pub const kAudio_UnimplementedError: OSStatus = -4;
pub const kAudio_FileNotFoundError: OSStatus = -43;
pub const kAudio_FilePermissionError: OSStatus = -54;
pub const kAudio_TooManyFilesOpenError: OSStatus = -42;
pub const kAudio_BadFilePathError: OSStatus = i32::from_be_bytes(*b"!pth");
pub const kAudio_ParamError: OSStatus = -50;
pub const kAudio_MemFullError: OSStatus = -108;

// CoreAudioBaseTypes.h

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioBuffer {
    pub mNumberChannels: u32,
    pub mDataByteSize: u32,
    pub mData: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioBufferList {
    mNumberBuffers: u32,
    mBuffers: [AudioBuffer; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioStreamPacketDescription {
    pub mStartOffset: i64,
    pub mVariableFramesInPacket: u32,
    pub mDataByteSize: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioStreamBasicDescription {
    pub mSampleRate: f64,
    pub mFormatID: AudioFormatID,
    pub mFormatFlags: AudioFormatFlags,
    pub mBytesPerPacket: u32,
    pub mFramesPerPacket: u32,
    pub mBytesPerFrame: u32,
    pub mChannelsPerFrame: u32,
    pub mBitsPerChannel: u32,
    pub mReserved: u32,
}

pub type AudioFormatID = u32;
pub type AudioFormatFlags = u32;

pub const kAudioFormatLinearPCM: AudioFormatID = 0x6c70636d;
pub const kAudioFormatAC3: AudioFormatID = 0x61632d33;
pub const kAudioFormat60958AC3: AudioFormatID = 0x63616333;
pub const kAudioFormatAppleIMA4: AudioFormatID = 0x696d6134;
pub const kAudioFormatMPEG4AAC: AudioFormatID = 0x61616320;
pub const kAudioFormatMPEG4CELP: AudioFormatID = 0x63656c70;
pub const kAudioFormatMPEG4HVXC: AudioFormatID = 0x68767863;
pub const kAudioFormatMPEG4TwinVQ: AudioFormatID = 0x74777671;
pub const kAudioFormatMACE3: AudioFormatID = 0x4d414333;
pub const kAudioFormatMACE6: AudioFormatID = 0x4d414336;
pub const kAudioFormatULaw: AudioFormatID = 0x756c6177;
pub const kAudioFormatALaw: AudioFormatID = 0x616c6177;
pub const kAudioFormatQDesign: AudioFormatID = 0x51444d43;
pub const kAudioFormatQDesign2: AudioFormatID = 0x51444d32;
pub const kAudioFormatQUALCOMM: AudioFormatID = 0x51636c70;
pub const kAudioFormatMPEGLayer1: AudioFormatID = 0x2e6d7031;
pub const kAudioFormatMPEGLayer2: AudioFormatID = 0x2e6d7032;
pub const kAudioFormatMPEGLayer3: AudioFormatID = 0x2e6d7033;
pub const kAudioFormatTimeCode: AudioFormatID = 0x74696d65;
pub const kAudioFormatMIDIStream: AudioFormatID = 0x6d696469;
pub const kAudioFormatParameterValueStream: AudioFormatID = 0x61707673;
pub const kAudioFormatAppleLossless: AudioFormatID = 0x616c6163;
pub const kAudioFormatMPEG4AAC_HE: AudioFormatID = 0x61616368;
pub const kAudioFormatMPEG4AAC_LD: AudioFormatID = 0x6161636c;
pub const kAudioFormatMPEG4AAC_ELD: AudioFormatID = 0x61616365;
pub const kAudioFormatMPEG4AAC_ELD_SBR: AudioFormatID = 0x61616366;
pub const kAudioFormatMPEG4AAC_ELD_V2: AudioFormatID = 0x61616367;
pub const kAudioFormatMPEG4AAC_HE_V2: AudioFormatID = 0x61616370;
pub const kAudioFormatMPEG4AAC_Spatial: AudioFormatID = 0x61616373;
pub const kAudioFormatMPEGD_USAC: AudioFormatID = 0x75736163;
pub const kAudioFormatAMR: AudioFormatID = 0x73616d72;
pub const kAudioFormatAMR_WB: AudioFormatID = 0x73617762;
pub const kAudioFormatAudible: AudioFormatID = 0x41554442;
pub const kAudioFormatiLBC: AudioFormatID = 0x696c6263;
pub const kAudioFormatDVIIntelIMA: AudioFormatID = 0x6D730011;
pub const kAudioFormatMicrosoftGSM: AudioFormatID = 0x6D730031;
pub const kAudioFormatAES3: AudioFormatID = 0x61657333;
pub const kAudioFormatEnhancedAC3: AudioFormatID = 0x65632d33;
pub const kAudioFormatFLAC: AudioFormatID = 0x666c6163;
pub const kAudioFormatOpus: AudioFormatID = 0x6f707573;
pub const kAudioFormatAPAC: AudioFormatID = 0x61706163;

pub const kAudioFormatFlagIsFloat: AudioFormatFlags = 1 << 0;
pub const kAudioFormatFlagIsBigEndian: AudioFormatFlags = 1 << 1;
pub const kAudioFormatFlagIsSignedInteger: AudioFormatFlags = 1 << 2;
pub const kAudioFormatFlagIsPacked: AudioFormatFlags = 1 << 3;
pub const kAudioFormatFlagIsAlignedHigh: AudioFormatFlags = 1 << 4;
pub const kAudioFormatFlagIsNonInterleaved: AudioFormatFlags = 1 << 5;
pub const kAudioFormatFlagIsNonMixable: AudioFormatFlags = 1 << 6;
pub const kAudioFormatFlagsAreAllClear: AudioFormatFlags = 0x80000000;

pub const kLinearPCMFormatFlagIsFloat: AudioFormatFlags = kAudioFormatFlagIsFloat;
pub const kLinearPCMFormatFlagIsBigEndian: AudioFormatFlags = kAudioFormatFlagIsBigEndian;
pub const kLinearPCMFormatFlagIsSignedInteger: AudioFormatFlags = kAudioFormatFlagIsSignedInteger;
pub const kLinearPCMFormatFlagIsPacked: AudioFormatFlags = kAudioFormatFlagIsPacked;
pub const kLinearPCMFormatFlagIsAlignedHigh: AudioFormatFlags = kAudioFormatFlagIsAlignedHigh;
pub const kLinearPCMFormatFlagIsNonInterleaved: AudioFormatFlags = kAudioFormatFlagIsNonInterleaved;
pub const kLinearPCMFormatFlagIsNonMixable: AudioFormatFlags = kAudioFormatFlagIsNonMixable;
pub const kLinearPCMFormatFlagsSampleFractionShift: AudioFormatFlags = 7;
pub const kLinearPCMFormatFlagsSampleFractionMask: AudioFormatFlags =
    0x3F << kLinearPCMFormatFlagsSampleFractionShift;
pub const kLinearPCMFormatFlagsAreAllClear: AudioFormatFlags = kAudioFormatFlagsAreAllClear;

pub const kAppleLosslessFormatFlag_16BitSourceData: AudioFormatFlags = 1;
pub const kAppleLosslessFormatFlag_20BitSourceData: AudioFormatFlags = 2;
pub const kAppleLosslessFormatFlag_24BitSourceData: AudioFormatFlags = 3;
pub const kAppleLosslessFormatFlag_32BitSourceData: AudioFormatFlags = 4;

pub const kAudioFormatFlagsNativeEndian: AudioFormatFlags = 0;
pub const kAudioFormatFlagsNativeFloatPacked: AudioFormatFlags =
    kAudioFormatFlagIsFloat | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked;

pub type SMPTETimeType = u32;

pub const kSMPTETimeType24: SMPTETimeType = 0;
pub const kSMPTETimeType25: SMPTETimeType = 1;
pub const kSMPTETimeType30Drop: SMPTETimeType = 2;
pub const kSMPTETimeType30: SMPTETimeType = 3;
pub const kSMPTETimeType2997: SMPTETimeType = 4;
pub const kSMPTETimeType2997Drop: SMPTETimeType = 5;
pub const kSMPTETimeType60: SMPTETimeType = 6;
pub const kSMPTETimeType5994: SMPTETimeType = 7;
pub const kSMPTETimeType60Drop: SMPTETimeType = 8;
pub const kSMPTETimeType5994Drop: SMPTETimeType = 9;
pub const kSMPTETimeType50: SMPTETimeType = 10;
pub const kSMPTETimeType2398: SMPTETimeType = 11;

pub type SMPTETimeFlags = u32;

pub const kSMPTETimeUnknown: SMPTETimeFlags = 0;
pub const kSMPTETimeValid: SMPTETimeFlags = 1 << 0;
pub const kSMPTETimeRunning: SMPTETimeFlags = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMPTETime {
    pub mSubframes: i16,
    pub mSubframeDivisor: i16,
    pub mCounter: u32,
    pub mType: SMPTETimeType,
    pub mFlags: SMPTETimeFlags,
    pub mHours: i16,
    pub mMinutes: i16,
    pub mSeconds: i16,
    pub mFrames: i16,
}

pub type AudioTimeStampFlags = u32;

pub const kAudioTimeStampNothingValid: AudioTimeStampFlags = 0;
pub const kAudioTimeStampSampleTimeValid: AudioTimeStampFlags = 1 << 0;
pub const kAudioTimeStampHostTimeValid: AudioTimeStampFlags = 1 << 1;
pub const kAudioTimeStampRateScalarValid: AudioTimeStampFlags = 1 << 2;
pub const kAudioTimeStampWordClockTimeValid: AudioTimeStampFlags = 1 << 3;
pub const kAudioTimeStampSMPTETimeValid: AudioTimeStampFlags = 1 << 4;
pub const kAudioTimeStampSampleHostTimeValid: AudioTimeStampFlags =
    kAudioTimeStampSampleTimeValid | kAudioTimeStampHostTimeValid;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioTimeStamp {
    pub mSampleTime: f64,
    pub mHostTime: u64,
    pub mRateScalar: f64,
    pub mWordClockTime: u64,
    pub mSMPTETime: SMPTETime,
    pub mFlags: AudioTimeStampFlags,
    pub mReserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioClassDescription {
    pub mType: i32,
    pub mSubType: i32,
    pub mManufacturer: i32,
}

pub type AudioChannelLabel = u32;

pub const kAudioChannelLabel_Unknown: AudioChannelLabel = 0xFFFFFFFF;
pub const kAudioChannelLabel_Unused: AudioChannelLabel = 0;
pub const kAudioChannelLabel_UseCoordinates: AudioChannelLabel = 100;
pub const kAudioChannelLabel_Left: AudioChannelLabel = 1;
pub const kAudioChannelLabel_Right: AudioChannelLabel = 2;
pub const kAudioChannelLabel_Center: AudioChannelLabel = 3;
pub const kAudioChannelLabel_LFEScreen: AudioChannelLabel = 4;
pub const kAudioChannelLabel_LeftSurround: AudioChannelLabel = 5;
pub const kAudioChannelLabel_RightSurround: AudioChannelLabel = 6;
pub const kAudioChannelLabel_LeftCenter: AudioChannelLabel = 7;
pub const kAudioChannelLabel_RightCenter: AudioChannelLabel = 8;
pub const kAudioChannelLabel_CenterSurround: AudioChannelLabel = 9;
pub const kAudioChannelLabel_LeftSurroundDirect: AudioChannelLabel = 10;
pub const kAudioChannelLabel_RightSurroundDirect: AudioChannelLabel = 11;
pub const kAudioChannelLabel_TopCenterSurround: AudioChannelLabel = 12;
pub const kAudioChannelLabel_VerticalHeightLeft: AudioChannelLabel = 13;
pub const kAudioChannelLabel_VerticalHeightCenter: AudioChannelLabel = 14;
pub const kAudioChannelLabel_VerticalHeightRight: AudioChannelLabel = 15;
pub const kAudioChannelLabel_TopBackLeft: AudioChannelLabel = 16;
pub const kAudioChannelLabel_TopBackCenter: AudioChannelLabel = 17;
pub const kAudioChannelLabel_TopBackRight: AudioChannelLabel = 18;
pub const kAudioChannelLabel_RearSurroundLeft: AudioChannelLabel = 33;
pub const kAudioChannelLabel_RearSurroundRight: AudioChannelLabel = 34;
pub const kAudioChannelLabel_LeftWide: AudioChannelLabel = 35;
pub const kAudioChannelLabel_RightWide: AudioChannelLabel = 36;
pub const kAudioChannelLabel_LFE2: AudioChannelLabel = 37;
pub const kAudioChannelLabel_LeftTotal: AudioChannelLabel = 38;
pub const kAudioChannelLabel_RightTotal: AudioChannelLabel = 39;
pub const kAudioChannelLabel_HearingImpaired: AudioChannelLabel = 40;
pub const kAudioChannelLabel_Narration: AudioChannelLabel = 41;
pub const kAudioChannelLabel_Mono: AudioChannelLabel = 42;
pub const kAudioChannelLabel_DialogCentricMix: AudioChannelLabel = 43;
pub const kAudioChannelLabel_CenterSurroundDirect: AudioChannelLabel = 44;
pub const kAudioChannelLabel_Haptic: AudioChannelLabel = 45;
pub const kAudioChannelLabel_LeftTopFront: AudioChannelLabel =
    kAudioChannelLabel_VerticalHeightLeft;
pub const kAudioChannelLabel_CenterTopFront: AudioChannelLabel =
    kAudioChannelLabel_VerticalHeightCenter;
pub const kAudioChannelLabel_RightTopFront: AudioChannelLabel =
    kAudioChannelLabel_VerticalHeightRight;
pub const kAudioChannelLabel_LeftTopMiddle: AudioChannelLabel = 49;
pub const kAudioChannelLabel_CenterTopMiddle: AudioChannelLabel =
    kAudioChannelLabel_TopCenterSurround;
pub const kAudioChannelLabel_RightTopMiddle: AudioChannelLabel = 51;
pub const kAudioChannelLabel_LeftTopRear: AudioChannelLabel = 52;
pub const kAudioChannelLabel_CenterTopRear: AudioChannelLabel = 53;
pub const kAudioChannelLabel_RightTopRear: AudioChannelLabel = 54;
pub const kAudioChannelLabel_Ambisonic_W: AudioChannelLabel = 200;
pub const kAudioChannelLabel_Ambisonic_X: AudioChannelLabel = 201;
pub const kAudioChannelLabel_Ambisonic_Y: AudioChannelLabel = 202;
pub const kAudioChannelLabel_Ambisonic_Z: AudioChannelLabel = 203;
pub const kAudioChannelLabel_MS_Mid: AudioChannelLabel = 204;
pub const kAudioChannelLabel_MS_Side: AudioChannelLabel = 205;
pub const kAudioChannelLabel_XY_X: AudioChannelLabel = 206;
pub const kAudioChannelLabel_XY_Y: AudioChannelLabel = 207;
pub const kAudioChannelLabel_BinauralLeft: AudioChannelLabel = 208;
pub const kAudioChannelLabel_BinauralRight: AudioChannelLabel = 209;
pub const kAudioChannelLabel_HeadphonesLeft: AudioChannelLabel = 301;
pub const kAudioChannelLabel_HeadphonesRight: AudioChannelLabel = 302;
pub const kAudioChannelLabel_ClickTrack: AudioChannelLabel = 304;
pub const kAudioChannelLabel_ForeignLanguage: AudioChannelLabel = 305;
pub const kAudioChannelLabel_Discrete: AudioChannelLabel = 400;
pub const kAudioChannelLabel_Discrete_0: AudioChannelLabel = (1 << 16) | 0;
pub const kAudioChannelLabel_Discrete_1: AudioChannelLabel = (1 << 16) | 1;
pub const kAudioChannelLabel_Discrete_2: AudioChannelLabel = (1 << 16) | 2;
pub const kAudioChannelLabel_Discrete_3: AudioChannelLabel = (1 << 16) | 3;
pub const kAudioChannelLabel_Discrete_4: AudioChannelLabel = (1 << 16) | 4;
pub const kAudioChannelLabel_Discrete_5: AudioChannelLabel = (1 << 16) | 5;
pub const kAudioChannelLabel_Discrete_6: AudioChannelLabel = (1 << 16) | 6;
pub const kAudioChannelLabel_Discrete_7: AudioChannelLabel = (1 << 16) | 7;
pub const kAudioChannelLabel_Discrete_8: AudioChannelLabel = (1 << 16) | 8;
pub const kAudioChannelLabel_Discrete_9: AudioChannelLabel = (1 << 16) | 9;
pub const kAudioChannelLabel_Discrete_10: AudioChannelLabel = (1 << 16) | 10;
pub const kAudioChannelLabel_Discrete_11: AudioChannelLabel = (1 << 16) | 11;
pub const kAudioChannelLabel_Discrete_12: AudioChannelLabel = (1 << 16) | 12;
pub const kAudioChannelLabel_Discrete_13: AudioChannelLabel = (1 << 16) | 13;
pub const kAudioChannelLabel_Discrete_14: AudioChannelLabel = (1 << 16) | 14;
pub const kAudioChannelLabel_Discrete_15: AudioChannelLabel = (1 << 16) | 15;
pub const kAudioChannelLabel_Discrete_65535: AudioChannelLabel = (1 << 16) | 65535;
pub const kAudioChannelLabel_HOA_ACN: AudioChannelLabel = 500;
pub const kAudioChannelLabel_HOA_ACN_0: AudioChannelLabel = (2 << 16) | 0;
pub const kAudioChannelLabel_HOA_ACN_1: AudioChannelLabel = (2 << 16) | 1;
pub const kAudioChannelLabel_HOA_ACN_2: AudioChannelLabel = (2 << 16) | 2;
pub const kAudioChannelLabel_HOA_ACN_3: AudioChannelLabel = (2 << 16) | 3;
pub const kAudioChannelLabel_HOA_ACN_4: AudioChannelLabel = (2 << 16) | 4;
pub const kAudioChannelLabel_HOA_ACN_5: AudioChannelLabel = (2 << 16) | 5;
pub const kAudioChannelLabel_HOA_ACN_6: AudioChannelLabel = (2 << 16) | 6;
pub const kAudioChannelLabel_HOA_ACN_7: AudioChannelLabel = (2 << 16) | 7;
pub const kAudioChannelLabel_HOA_ACN_8: AudioChannelLabel = (2 << 16) | 8;
pub const kAudioChannelLabel_HOA_ACN_9: AudioChannelLabel = (2 << 16) | 9;
pub const kAudioChannelLabel_HOA_ACN_10: AudioChannelLabel = (2 << 16) | 10;
pub const kAudioChannelLabel_HOA_ACN_11: AudioChannelLabel = (2 << 16) | 11;
pub const kAudioChannelLabel_HOA_ACN_12: AudioChannelLabel = (2 << 16) | 12;
pub const kAudioChannelLabel_HOA_ACN_13: AudioChannelLabel = (2 << 16) | 13;
pub const kAudioChannelLabel_HOA_ACN_14: AudioChannelLabel = (2 << 16) | 14;
pub const kAudioChannelLabel_HOA_ACN_15: AudioChannelLabel = (2 << 16) | 15;
pub const kAudioChannelLabel_HOA_ACN_65024: AudioChannelLabel = (2 << 16) | 65024;
pub const kAudioChannelLabel_BeginReserved: AudioChannelLabel = 0xF0000000;
pub const kAudioChannelLabel_EndReserved: AudioChannelLabel = 0xFFFFFFFE;

pub type AudioChannelBitmap = u32;

pub const kAudioChannelBit_Left: AudioChannelBitmap = 1 << 0;
pub const kAudioChannelBit_Right: AudioChannelBitmap = 1 << 1;
pub const kAudioChannelBit_Center: AudioChannelBitmap = 1 << 2;
pub const kAudioChannelBit_LFEScreen: AudioChannelBitmap = 1 << 3;
pub const kAudioChannelBit_LeftSurround: AudioChannelBitmap = 1 << 4;
pub const kAudioChannelBit_RightSurround: AudioChannelBitmap = 1 << 5;
pub const kAudioChannelBit_LeftCenter: AudioChannelBitmap = 1 << 6;
pub const kAudioChannelBit_RightCenter: AudioChannelBitmap = 1 << 7;
pub const kAudioChannelBit_CenterSurround: AudioChannelBitmap = 1 << 8;
pub const kAudioChannelBit_LeftSurroundDirect: AudioChannelBitmap = 1 << 9;
pub const kAudioChannelBit_RightSurroundDirect: AudioChannelBitmap = 1 << 10;
pub const kAudioChannelBit_TopCenterSurround: AudioChannelBitmap = 1 << 11;
pub const kAudioChannelBit_VerticalHeightLeft: AudioChannelBitmap = 1 << 12;
pub const kAudioChannelBit_VerticalHeightCenter: AudioChannelBitmap = 1 << 13;
pub const kAudioChannelBit_VerticalHeightRight: AudioChannelBitmap = 1 << 14;
pub const kAudioChannelBit_TopBackLeft: AudioChannelBitmap = 1 << 15;
pub const kAudioChannelBit_TopBackCenter: AudioChannelBitmap = 1 << 16;
pub const kAudioChannelBit_TopBackRight: AudioChannelBitmap = 1 << 17;
pub const kAudioChannelBit_LeftTopFront: AudioChannelBitmap = kAudioChannelBit_VerticalHeightLeft;
pub const kAudioChannelBit_CenterTopFront: AudioChannelBitmap =
    kAudioChannelBit_VerticalHeightCenter;
pub const kAudioChannelBit_RightTopFront: AudioChannelBitmap = kAudioChannelBit_VerticalHeightRight;
pub const kAudioChannelBit_LeftTopMiddle: AudioChannelBitmap = 1 << 21;
pub const kAudioChannelBit_CenterTopMiddle: AudioChannelBitmap = kAudioChannelBit_TopCenterSurround;
pub const kAudioChannelBit_RightTopMiddle: AudioChannelBitmap = 1 << 23;
pub const kAudioChannelBit_LeftTopRear: AudioChannelBitmap = 1 << 24;
pub const kAudioChannelBit_CenterTopRear: AudioChannelBitmap = 1 << 25;
pub const kAudioChannelBit_RightTopRear: AudioChannelBitmap = 1 << 26;

pub type AudioChannelFlags = u32;

pub const kAudioChannelFlags_AllOff: AudioChannelFlags = 0;
pub const kAudioChannelFlags_RectangularCoordinates: AudioChannelFlags = 1 << 0;
pub const kAudioChannelFlags_SphericalCoordinates: AudioChannelFlags = 1 << 1;
pub const kAudioChannelFlags_Meters: AudioChannelFlags = 1 << 2;

pub type AudioChannelCoordinateIndex = u32;

pub const kAudioChannelCoordinates_LeftRight: AudioChannelCoordinateIndex = 0;
pub const kAudioChannelCoordinates_BackFront: AudioChannelCoordinateIndex = 1;
pub const kAudioChannelCoordinates_DownUp: AudioChannelCoordinateIndex = 2;
pub const kAudioChannelCoordinates_Azimuth: AudioChannelCoordinateIndex = 0;
pub const kAudioChannelCoordinates_Elevation: AudioChannelCoordinateIndex = 1;
pub const kAudioChannelCoordinates_Distance: AudioChannelCoordinateIndex = 2;

pub type AudioChannelLayoutTag = u32;

pub const kAudioChannelLayoutTag_UseChannelDescriptions: AudioChannelLayoutTag = (0 << 16) | 0;
pub const kAudioChannelLayoutTag_UseChannelBitmap: AudioChannelLayoutTag = (1 << 16) | 0;
pub const kAudioChannelLayoutTag_Mono: AudioChannelLayoutTag = (100 << 16) | 1;
pub const kAudioChannelLayoutTag_Stereo: AudioChannelLayoutTag = (101 << 16) | 2;
pub const kAudioChannelLayoutTag_StereoHeadphones: AudioChannelLayoutTag = (102 << 16) | 2;
pub const kAudioChannelLayoutTag_MatrixStereo: AudioChannelLayoutTag = (103 << 16) | 2;
pub const kAudioChannelLayoutTag_MidSide: AudioChannelLayoutTag = (104 << 16) | 2;
pub const kAudioChannelLayoutTag_XY: AudioChannelLayoutTag = (105 << 16) | 2;
pub const kAudioChannelLayoutTag_Binaural: AudioChannelLayoutTag = (106 << 16) | 2;
pub const kAudioChannelLayoutTag_Ambisonic_B_Format: AudioChannelLayoutTag = (107 << 16) | 4;
pub const kAudioChannelLayoutTag_Quadraphonic: AudioChannelLayoutTag = (108 << 16) | 4;
pub const kAudioChannelLayoutTag_Pentagonal: AudioChannelLayoutTag = (109 << 16) | 5;
pub const kAudioChannelLayoutTag_Hexagonal: AudioChannelLayoutTag = (110 << 16) | 6;
pub const kAudioChannelLayoutTag_Octagonal: AudioChannelLayoutTag = (111 << 16) | 8;
pub const kAudioChannelLayoutTag_Cube: AudioChannelLayoutTag = (112 << 16) | 8;
pub const kAudioChannelLayoutTag_MPEG_1_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Mono;
pub const kAudioChannelLayoutTag_MPEG_2_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Stereo;
pub const kAudioChannelLayoutTag_MPEG_3_0_A: AudioChannelLayoutTag = (113 << 16) | 3;
pub const kAudioChannelLayoutTag_MPEG_3_0_B: AudioChannelLayoutTag = (114 << 16) | 3;
pub const kAudioChannelLayoutTag_MPEG_4_0_A: AudioChannelLayoutTag = (115 << 16) | 4;
pub const kAudioChannelLayoutTag_MPEG_4_0_B: AudioChannelLayoutTag = (116 << 16) | 4;
pub const kAudioChannelLayoutTag_MPEG_5_0_A: AudioChannelLayoutTag = (117 << 16) | 5;
pub const kAudioChannelLayoutTag_MPEG_5_0_B: AudioChannelLayoutTag = (118 << 16) | 5;
pub const kAudioChannelLayoutTag_MPEG_5_0_C: AudioChannelLayoutTag = (119 << 16) | 5;
pub const kAudioChannelLayoutTag_MPEG_5_0_D: AudioChannelLayoutTag = (120 << 16) | 5;
pub const kAudioChannelLayoutTag_MPEG_5_1_A: AudioChannelLayoutTag = (121 << 16) | 6;
pub const kAudioChannelLayoutTag_MPEG_5_1_B: AudioChannelLayoutTag = (122 << 16) | 6;
pub const kAudioChannelLayoutTag_MPEG_5_1_C: AudioChannelLayoutTag = (123 << 16) | 6;
pub const kAudioChannelLayoutTag_MPEG_5_1_D: AudioChannelLayoutTag = (124 << 16) | 6;
pub const kAudioChannelLayoutTag_MPEG_6_1_A: AudioChannelLayoutTag = (125 << 16) | 7;
pub const kAudioChannelLayoutTag_MPEG_7_1_A: AudioChannelLayoutTag = (126 << 16) | 8;
pub const kAudioChannelLayoutTag_MPEG_7_1_B: AudioChannelLayoutTag = (127 << 16) | 8;
pub const kAudioChannelLayoutTag_MPEG_7_1_C: AudioChannelLayoutTag = (128 << 16) | 8;
pub const kAudioChannelLayoutTag_Emagic_Default_7_1: AudioChannelLayoutTag = (129 << 16) | 8;
pub const kAudioChannelLayoutTag_SMPTE_DTV: AudioChannelLayoutTag = (130 << 16) | 8;
pub const kAudioChannelLayoutTag_ITU_1_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Mono;
pub const kAudioChannelLayoutTag_ITU_2_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Stereo;
pub const kAudioChannelLayoutTag_ITU_2_1: AudioChannelLayoutTag = (131 << 16) | 3;
pub const kAudioChannelLayoutTag_ITU_2_2: AudioChannelLayoutTag = (132 << 16) | 4;
pub const kAudioChannelLayoutTag_ITU_3_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_3_0_A;
pub const kAudioChannelLayoutTag_ITU_3_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_4_0_A;
pub const kAudioChannelLayoutTag_ITU_3_2: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_A;
pub const kAudioChannelLayoutTag_ITU_3_2_1: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_5_1_A;
pub const kAudioChannelLayoutTag_ITU_3_4_1: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_7_1_C;
pub const kAudioChannelLayoutTag_DVD_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Mono;
pub const kAudioChannelLayoutTag_DVD_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_Stereo;
pub const kAudioChannelLayoutTag_DVD_2: AudioChannelLayoutTag = kAudioChannelLayoutTag_ITU_2_1;
pub const kAudioChannelLayoutTag_DVD_3: AudioChannelLayoutTag = kAudioChannelLayoutTag_ITU_2_2;
pub const kAudioChannelLayoutTag_DVD_4: AudioChannelLayoutTag = (133 << 16) | 3;
pub const kAudioChannelLayoutTag_DVD_5: AudioChannelLayoutTag = (134 << 16) | 4;
pub const kAudioChannelLayoutTag_DVD_6: AudioChannelLayoutTag = (135 << 16) | 5;
pub const kAudioChannelLayoutTag_DVD_7: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_3_0_A;
pub const kAudioChannelLayoutTag_DVD_8: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_4_0_A;
pub const kAudioChannelLayoutTag_DVD_9: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_A;
pub const kAudioChannelLayoutTag_DVD_10: AudioChannelLayoutTag = (136 << 16) | 4;
pub const kAudioChannelLayoutTag_DVD_11: AudioChannelLayoutTag = (137 << 16) | 5;
pub const kAudioChannelLayoutTag_DVD_12: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_A;
pub const kAudioChannelLayoutTag_DVD_13: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_8;
pub const kAudioChannelLayoutTag_DVD_14: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_9;
pub const kAudioChannelLayoutTag_DVD_15: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_10;
pub const kAudioChannelLayoutTag_DVD_16: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_11;
pub const kAudioChannelLayoutTag_DVD_17: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_12;
pub const kAudioChannelLayoutTag_DVD_18: AudioChannelLayoutTag = (138 << 16) | 5;
pub const kAudioChannelLayoutTag_DVD_19: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_B;
pub const kAudioChannelLayoutTag_DVD_20: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_B;
pub const kAudioChannelLayoutTag_AudioUnit_4: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_Quadraphonic;
pub const kAudioChannelLayoutTag_AudioUnit_5: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_Pentagonal;
pub const kAudioChannelLayoutTag_AudioUnit_6: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_Hexagonal;
pub const kAudioChannelLayoutTag_AudioUnit_8: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_Octagonal;
pub const kAudioChannelLayoutTag_AudioUnit_5_0: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_5_0_B;
pub const kAudioChannelLayoutTag_AudioUnit_6_0: AudioChannelLayoutTag = (139 << 16) | 6;
pub const kAudioChannelLayoutTag_AudioUnit_7_0: AudioChannelLayoutTag = (140 << 16) | 7;
pub const kAudioChannelLayoutTag_AudioUnit_7_0_Front: AudioChannelLayoutTag = (148 << 16) | 7;
pub const kAudioChannelLayoutTag_AudioUnit_5_1: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_5_1_A;
pub const kAudioChannelLayoutTag_AudioUnit_6_1: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_6_1_A;
pub const kAudioChannelLayoutTag_AudioUnit_7_1: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_7_1_C;
pub const kAudioChannelLayoutTag_AudioUnit_7_1_Front: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_7_1_A;
pub const kAudioChannelLayoutTag_AAC_3_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_3_0_B;
pub const kAudioChannelLayoutTag_AAC_Quadraphonic: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_Quadraphonic;
pub const kAudioChannelLayoutTag_AAC_4_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_4_0_B;
pub const kAudioChannelLayoutTag_AAC_5_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_D;
pub const kAudioChannelLayoutTag_AAC_5_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_D;
pub const kAudioChannelLayoutTag_AAC_6_0: AudioChannelLayoutTag = (141 << 16) | 6;
pub const kAudioChannelLayoutTag_AAC_6_1: AudioChannelLayoutTag = (142 << 16) | 7;
pub const kAudioChannelLayoutTag_AAC_7_0: AudioChannelLayoutTag = (143 << 16) | 7;
pub const kAudioChannelLayoutTag_AAC_7_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_7_1_B;
pub const kAudioChannelLayoutTag_AAC_7_1_B: AudioChannelLayoutTag = (183 << 16) | 8;
pub const kAudioChannelLayoutTag_AAC_7_1_C: AudioChannelLayoutTag = (184 << 16) | 8;
pub const kAudioChannelLayoutTag_AAC_Octagonal: AudioChannelLayoutTag = (144 << 16) | 8;
pub const kAudioChannelLayoutTag_TMH_10_2_std: AudioChannelLayoutTag = (145 << 16) | 16;
pub const kAudioChannelLayoutTag_TMH_10_2_full: AudioChannelLayoutTag = (146 << 16) | 21;
pub const kAudioChannelLayoutTag_AC3_1_0_1: AudioChannelLayoutTag = (149 << 16) | 2;
pub const kAudioChannelLayoutTag_AC3_3_0: AudioChannelLayoutTag = (150 << 16) | 3;
pub const kAudioChannelLayoutTag_AC3_3_1: AudioChannelLayoutTag = (151 << 16) | 4;
pub const kAudioChannelLayoutTag_AC3_3_0_1: AudioChannelLayoutTag = (152 << 16) | 4;
pub const kAudioChannelLayoutTag_AC3_2_1_1: AudioChannelLayoutTag = (153 << 16) | 4;
pub const kAudioChannelLayoutTag_AC3_3_1_1: AudioChannelLayoutTag = (154 << 16) | 5;
pub const kAudioChannelLayoutTag_EAC_6_0_A: AudioChannelLayoutTag = (155 << 16) | 6;
pub const kAudioChannelLayoutTag_EAC_7_0_A: AudioChannelLayoutTag = (156 << 16) | 7;
pub const kAudioChannelLayoutTag_EAC3_6_1_A: AudioChannelLayoutTag = (157 << 16) | 7;
pub const kAudioChannelLayoutTag_EAC3_6_1_B: AudioChannelLayoutTag = (158 << 16) | 7;
pub const kAudioChannelLayoutTag_EAC3_6_1_C: AudioChannelLayoutTag = (159 << 16) | 7;
pub const kAudioChannelLayoutTag_EAC3_7_1_A: AudioChannelLayoutTag = (160 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_B: AudioChannelLayoutTag = (161 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_C: AudioChannelLayoutTag = (162 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_D: AudioChannelLayoutTag = (163 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_E: AudioChannelLayoutTag = (164 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_F: AudioChannelLayoutTag = (165 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_G: AudioChannelLayoutTag = (166 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_H: AudioChannelLayoutTag = (167 << 16) | 8;
pub const kAudioChannelLayoutTag_DTS_3_1: AudioChannelLayoutTag = (168 << 16) | 4;
pub const kAudioChannelLayoutTag_DTS_4_1: AudioChannelLayoutTag = (169 << 16) | 5;
pub const kAudioChannelLayoutTag_DTS_6_0_A: AudioChannelLayoutTag = (170 << 16) | 6;
pub const kAudioChannelLayoutTag_DTS_6_0_B: AudioChannelLayoutTag = (171 << 16) | 6;
pub const kAudioChannelLayoutTag_DTS_6_0_C: AudioChannelLayoutTag = (172 << 16) | 6;
pub const kAudioChannelLayoutTag_DTS_6_1_A: AudioChannelLayoutTag = (173 << 16) | 7;
pub const kAudioChannelLayoutTag_DTS_6_1_B: AudioChannelLayoutTag = (174 << 16) | 7;
pub const kAudioChannelLayoutTag_DTS_6_1_C: AudioChannelLayoutTag = (175 << 16) | 7;
pub const kAudioChannelLayoutTag_DTS_7_0: AudioChannelLayoutTag = (176 << 16) | 7;
pub const kAudioChannelLayoutTag_DTS_7_1: AudioChannelLayoutTag = (177 << 16) | 8;
pub const kAudioChannelLayoutTag_DTS_8_0_A: AudioChannelLayoutTag = (178 << 16) | 8;
pub const kAudioChannelLayoutTag_DTS_8_0_B: AudioChannelLayoutTag = (179 << 16) | 8;
pub const kAudioChannelLayoutTag_DTS_8_1_A: AudioChannelLayoutTag = (180 << 16) | 9;
pub const kAudioChannelLayoutTag_DTS_8_1_B: AudioChannelLayoutTag = (181 << 16) | 9;
pub const kAudioChannelLayoutTag_WAVE_2_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_4;
pub const kAudioChannelLayoutTag_WAVE_3_0: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_3_0_A;
pub const kAudioChannelLayoutTag_WAVE_4_0_A: AudioChannelLayoutTag = kAudioChannelLayoutTag_ITU_2_2;
pub const kAudioChannelLayoutTag_WAVE_4_0_B: AudioChannelLayoutTag = (185 << 16) | 4;
pub const kAudioChannelLayoutTag_WAVE_5_0_A: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_5_0_A;
pub const kAudioChannelLayoutTag_WAVE_5_0_B: AudioChannelLayoutTag = (186 << 16) | 5;
pub const kAudioChannelLayoutTag_WAVE_5_1_A: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_5_1_A;
pub const kAudioChannelLayoutTag_WAVE_5_1_B: AudioChannelLayoutTag = (187 << 16) | 6;
pub const kAudioChannelLayoutTag_WAVE_6_1: AudioChannelLayoutTag = (188 << 16) | 7;
pub const kAudioChannelLayoutTag_WAVE_7_1: AudioChannelLayoutTag = (189 << 16) | 8;
pub const kAudioChannelLayoutTag_HOA_ACN_SN3D: AudioChannelLayoutTag = (190 << 16) | 0;
pub const kAudioChannelLayoutTag_HOA_ACN_N3D: AudioChannelLayoutTag = (191 << 16) | 0;
pub const kAudioChannelLayoutTag_Atmos_5_1_2: AudioChannelLayoutTag = (194 << 16) | 8;
pub const kAudioChannelLayoutTag_Atmos_5_1_4: AudioChannelLayoutTag = (195 << 16) | 10;
pub const kAudioChannelLayoutTag_Atmos_7_1_2: AudioChannelLayoutTag = (196 << 16) | 10;
pub const kAudioChannelLayoutTag_Atmos_7_1_4: AudioChannelLayoutTag = (192 << 16) | 12;
pub const kAudioChannelLayoutTag_Atmos_9_1_6: AudioChannelLayoutTag = (193 << 16) | 16;
pub const kAudioChannelLayoutTag_DiscreteInOrder: AudioChannelLayoutTag = (147 << 16) | 0;
pub const kAudioChannelLayoutTag_BeginReserved: AudioChannelLayoutTag = 0xF0000000;
pub const kAudioChannelLayoutTag_EndReserved: AudioChannelLayoutTag = 0xFFFEFFFF;
pub const kAudioChannelLayoutTag_Unknown: AudioChannelLayoutTag = 0xFFFF0000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioChannelDescription {
    pub mChannelLabel: AudioChannelLabel,
    pub mChannelFlags: AudioChannelFlags,
    pub mCoordinates: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioChannelLayout {
    pub mChannelLayoutTag: AudioChannelLayoutTag,
    pub mChannelBitmap: AudioChannelBitmap,
    pub mNumberChannelDescriptions: u32,
    pub mChannelDescriptions: [AudioChannelDescription; 0],
}

// MidiServices.h

pub type MIDITimeStamp = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDIPacket {
    pub timeStamp: MIDITimeStamp,
    pub length: u16,
    pub data: [u8; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDIPacketList {
    pub numPackets: u32,
    pub packet: [MIDIPacket; 0],
}

// AudioComponent.h

pub type AudioComponentFlags = u32;

pub const kAudioComponentFlag_Unsearchable: AudioComponentFlags = 1;
pub const kAudioComponentFlag_SandboxSafe: AudioComponentFlags = 2;
pub const kAudioComponentFlag_IsV3AudioUnit: AudioComponentFlags = 4;
pub const kAudioComponentFlag_RequiresAsyncInstantiation: AudioComponentFlags = 8;
pub const kAudioComponentFlag_CanLoadInProcess: AudioComponentFlags = 0x10;

pub type AudioComponentInstantiationOptions = u32;

pub const kAudioComponentInstantiation_LoadOutOfProcess: AudioComponentInstantiationOptions = 1;
pub const kAudioComponentInstantiation_LoadInProcess: AudioComponentInstantiationOptions = 2;
pub const kAudioComponentInstantiation_LoadedRemotely: AudioComponentInstantiationOptions = 1 << 31;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioComponentDescription {
    pub componentType: i32,
    pub componentSubType: i32,
    pub componentManufacturer: i32,
    pub componentFlags: u32,
    pub componentFlagsMask: u32,
}

pub enum AudioComponent__ {}
pub type AudioComponent = *mut AudioComponent__;

pub enum AudioComponentInstance__ {}
pub type AudioComponentInstance = *mut AudioComponentInstance__;

pub type AudioComponentMethod = unsafe extern "C" fn(self_: *mut c_void, ...) -> OSStatus;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioComponentPlugInInterface {
    pub Open: Option<
        unsafe extern "C" fn(self_: *mut c_void, mInstance: AudioComponentInstance) -> OSStatus,
    >,
    pub Close: Option<unsafe extern "C" fn(self_: *mut c_void) -> OSStatus>,
    pub Lookup: Option<unsafe extern "C" fn(selector: i16) -> Option<AudioComponentMethod>>,
    pub reserved: *mut c_void,
}

pub type AudioComponentFactoryFunction = unsafe extern "C" fn(
    inDesc: *const AudioComponentDescription,
)
    -> *mut AudioComponentPlugInInterface;

#[cfg_attr(target_os = "macos", link(name = "AudioToolbox", kind = "framework"))]
unsafe extern "C" {
    pub fn AudioComponentFindNext(
        inComponent: AudioComponent,
        inDesc: *const AudioComponentDescription,
    ) -> AudioComponent;

    pub fn AudioComponentCount(inDesc: *const AudioComponentDescription) -> u32;

    pub fn AudioComponentCopyName(
        inComponent: AudioComponent,
        outName: *mut CFStringRef,
    ) -> OSStatus;

    pub fn AudioComponentGetDescription(
        inComponent: AudioComponent,
        outDesc: *mut AudioComponentDescription,
    ) -> OSStatus;

    pub fn AudioComponentGetVersion(inComponent: AudioComponent, outVersion: *mut u32) -> OSStatus;

    pub fn AudioComponentInstanceNew(
        inComponent: AudioComponent,
        outInstance: *mut AudioComponentInstance,
    ) -> OSStatus;

    pub fn AudioComponentInstanceDispose(inInstance: AudioComponentInstance) -> OSStatus;

    pub fn AudioComponentInstanceGetComponent(inInstance: AudioComponentInstance)
    -> AudioComponent;
    pub fn AudioComponentInstanceCanDo(
        inInstance: AudioComponentInstance,
        inSelectorID: i16,
    ) -> Boolean;

    pub fn AudioComponentRegister(
        inDesc: *const AudioComponentDescription,
        inName: CFStringRef,
        inVersion: u32,
        inFactory: AudioComponentFactoryFunction,
    ) -> AudioComponent;

    pub fn AudioComponentCopyConfigurationInfo(
        inComponent: AudioComponent,
        outConfigurationInfo: *mut CFDictionaryRef,
    ) -> OSStatus;
}

// AUComponent.h

pub type AudioUnit = AudioComponentInstance;

pub const kAudioUnitType_Output: u32 = u32::from_be_bytes(*b"auou");
pub const kAudioUnitType_MusicDevice: u32 = u32::from_be_bytes(*b"aumu");
pub const kAudioUnitType_MusicEffect: u32 = u32::from_be_bytes(*b"aumf");
pub const kAudioUnitType_FormatConverter: u32 = u32::from_be_bytes(*b"aufc");
pub const kAudioUnitType_Effect: u32 = u32::from_be_bytes(*b"aufx");
pub const kAudioUnitType_Mixer: u32 = u32::from_be_bytes(*b"aumx");
pub const kAudioUnitType_Panner: u32 = u32::from_be_bytes(*b"aupn");
pub const kAudioUnitType_Generator: u32 = u32::from_be_bytes(*b"augn");
pub const kAudioUnitType_OfflineEffect: u32 = u32::from_be_bytes(*b"auol");
pub const kAudioUnitType_MIDIProcessor: u32 = u32::from_be_bytes(*b"aumi");
pub const kAudioUnitType_RemoteEffect: u32 = u32::from_be_bytes(*b"aurx");
pub const kAudioUnitType_RemoteGenerator: u32 = u32::from_be_bytes(*b"aurg");
pub const kAudioUnitType_RemoteInstrument: u32 = u32::from_be_bytes(*b"auri");
pub const kAudioUnitType_RemoteMusicEffect: u32 = u32::from_be_bytes(*b"aurm");

pub const kAudioUnitManufacturer_Apple: u32 = u32::from_be_bytes(*b"appl");

pub const kAudioUnitSubType_GenericOutput: u32 = u32::from_be_bytes(*b"genr");
pub const kAudioUnitSubType_VoiceProcessingIO: u32 = u32::from_be_bytes(*b"vpio");
pub const kAudioUnitSubType_HALOutput: u32 = u32::from_be_bytes(*b"ahal");
pub const kAudioUnitSubType_DefaultOutput: u32 = u32::from_be_bytes(*b"def ");
pub const kAudioUnitSubType_SystemOutput: u32 = u32::from_be_bytes(*b"sys ");
pub const kAudioUnitSubType_RemoteIO: u32 = u32::from_be_bytes(*b"rioc");
pub const kAudioUnitSubType_DLSSynth: u32 = u32::from_be_bytes(*b"dls ");
pub const kAudioUnitSubType_Sampler: u32 = u32::from_be_bytes(*b"samp");
pub const kAudioUnitSubType_MIDISynth: u32 = u32::from_be_bytes(*b"msyn");
pub const kAudioUnitSubType_AUConverter: u32 = u32::from_be_bytes(*b"conv");
pub const kAudioUnitSubType_Varispeed: u32 = u32::from_be_bytes(*b"vari");
pub const kAudioUnitSubType_DeferredRenderer: u32 = u32::from_be_bytes(*b"defr");
pub const kAudioUnitSubType_Splitter: u32 = u32::from_be_bytes(*b"splt");
pub const kAudioUnitSubType_MultiSplitter: u32 = u32::from_be_bytes(*b"mspl");
pub const kAudioUnitSubType_Merger: u32 = u32::from_be_bytes(*b"merg");
pub const kAudioUnitSubType_NewTimePitch: u32 = u32::from_be_bytes(*b"nutp");
pub const kAudioUnitSubType_AUiPodTimeOther: u32 = u32::from_be_bytes(*b"ipto");
pub const kAudioUnitSubType_RoundTripAAC: u32 = u32::from_be_bytes(*b"raac");
pub const kAudioUnitSubType_TimePitch: u32 = u32::from_be_bytes(*b"tmpt");
pub const kAudioUnitSubType_PeakLimiter: u32 = u32::from_be_bytes(*b"lmtr");
pub const kAudioUnitSubType_DynamicsProcessor: u32 = u32::from_be_bytes(*b"dcmp");
pub const kAudioUnitSubType_LowPassFilter: u32 = u32::from_be_bytes(*b"lpas");
pub const kAudioUnitSubType_HighPassFilter: u32 = u32::from_be_bytes(*b"hpas");
pub const kAudioUnitSubType_BandPassFilter: u32 = u32::from_be_bytes(*b"bpas");
pub const kAudioUnitSubType_HighShelfFilter: u32 = u32::from_be_bytes(*b"hshf");
pub const kAudioUnitSubType_LowShelfFilter: u32 = u32::from_be_bytes(*b"lshf");
pub const kAudioUnitSubType_ParametricEQ: u32 = u32::from_be_bytes(*b"pmeq");
pub const kAudioUnitSubType_Distortion: u32 = u32::from_be_bytes(*b"dist");
pub const kAudioUnitSubType_Delay: u32 = u32::from_be_bytes(*b"dely");
pub const kAudioUnitSubType_SampleDelay: u32 = u32::from_be_bytes(*b"sdly");
pub const kAudioUnitSubType_NBandEQ: u32 = u32::from_be_bytes(*b"nbeq");
pub const kAudioUnitSubType_Reverb2: u32 = u32::from_be_bytes(*b"rvb2");
pub const kAudioUnitSubType_GraphicEQ: u32 = u32::from_be_bytes(*b"greq");
pub const kAudioUnitSubType_MultiBandCompressor: u32 = u32::from_be_bytes(*b"mcmp");
pub const kAudioUnitSubType_MatrixReverb: u32 = u32::from_be_bytes(*b"mrev");
pub const kAudioUnitSubType_Pitch: u32 = u32::from_be_bytes(*b"tmpt");
pub const kAudioUnitSubType_AUFilter: u32 = u32::from_be_bytes(*b"filt");
pub const kAudioUnitSubType_NetSend: u32 = u32::from_be_bytes(*b"nsnd");
pub const kAudioUnitSubType_RogerBeep: u32 = u32::from_be_bytes(*b"rogr");
pub const kAudioUnitSubType_MultiChannelMixer: u32 = u32::from_be_bytes(*b"mcmx");
pub const kAudioUnitSubType_MatrixMixer: u32 = u32::from_be_bytes(*b"mxmx");
pub const kAudioUnitSubType_SpatialMixer: u32 = u32::from_be_bytes(*b"3dem");
pub const kAudioUnitSubType_StereoMixer: u32 = u32::from_be_bytes(*b"smxr");
pub const kAudioUnitSubType_SphericalHeadPanner: u32 = u32::from_be_bytes(*b"sphr");
pub const kAudioUnitSubType_VectorPanner: u32 = u32::from_be_bytes(*b"vbas");
pub const kAudioUnitSubType_SoundFieldPanner: u32 = u32::from_be_bytes(*b"ambi");
pub const kAudioUnitSubType_HRTFPanner: u32 = u32::from_be_bytes(*b"hrtf");
pub const kAudioUnitSubType_NetReceive: u32 = u32::from_be_bytes(*b"nrcv");
pub const kAudioUnitSubType_ScheduledSoundPlayer: u32 = u32::from_be_bytes(*b"sspl");
pub const kAudioUnitSubType_AudioFilePlayer: u32 = u32::from_be_bytes(*b"afpl");

pub type AudioUnitRenderActionFlags = u32;

pub const kAudioUnitRenderAction_PreRender: AudioUnitRenderActionFlags = 1 << 2;
pub const kAudioUnitRenderAction_PostRender: AudioUnitRenderActionFlags = 1 << 3;
pub const kAudioUnitRenderAction_OutputIsSilence: AudioUnitRenderActionFlags = 1 << 4;
pub const kAudioOfflineUnitRenderAction_Preflight: AudioUnitRenderActionFlags = 1 << 5;
pub const kAudioOfflineUnitRenderAction_Render: AudioUnitRenderActionFlags = 1 << 6;
pub const kAudioOfflineUnitRenderAction_Complete: AudioUnitRenderActionFlags = 1 << 7;
pub const kAudioUnitRenderAction_PostRenderError: AudioUnitRenderActionFlags = 1 << 8;
pub const kAudioUnitRenderAction_DoNotCheckRenderArgs: AudioUnitRenderActionFlags = 1 << 9;

pub const kAudioUnitErr_InvalidProperty: OSStatus = -10879;
pub const kAudioUnitErr_InvalidParameter: OSStatus = -10878;
pub const kAudioUnitErr_InvalidElement: OSStatus = -10877;
pub const kAudioUnitErr_NoConnection: OSStatus = -10876;
pub const kAudioUnitErr_FailedInitialization: OSStatus = -10875;
pub const kAudioUnitErr_TooManyFramesToProcess: OSStatus = -10874;
pub const kAudioUnitErr_InvalidFile: OSStatus = -10871;
pub const kAudioUnitErr_UnknownFileType: OSStatus = -10870;
pub const kAudioUnitErr_FileNotSpecified: OSStatus = -10869;
pub const kAudioUnitErr_FormatNotSupported: OSStatus = -10868;
pub const kAudioUnitErr_Uninitialized: OSStatus = -10867;
pub const kAudioUnitErr_InvalidScope: OSStatus = -10866;
pub const kAudioUnitErr_PropertyNotWritable: OSStatus = -10865;
pub const kAudioUnitErr_CannotDoInCurrentContext: OSStatus = -10863;
pub const kAudioUnitErr_InvalidPropertyValue: OSStatus = -10851;
pub const kAudioUnitErr_PropertyNotInUse: OSStatus = -10850;
pub const kAudioUnitErr_Initialized: OSStatus = -10849;
pub const kAudioUnitErr_InvalidOfflineRender: OSStatus = -10848;
pub const kAudioUnitErr_Unauthorized: OSStatus = -10847;
pub const kAudioUnitErr_MIDIOutputBufferFull: OSStatus = -66753;
pub const kAudioComponentErr_InstanceTimedOut: OSStatus = -66754;
pub const kAudioComponentErr_InstanceInvalidated: OSStatus = -66749;
pub const kAudioUnitErr_RenderTimeout: OSStatus = -66745;
pub const kAudioUnitErr_ExtensionNotFound: OSStatus = -66744;
pub const kAudioUnitErr_InvalidParameterValue: OSStatus = -66743;
pub const kAudioUnitErr_InvalidFilePath: OSStatus = -66742;
pub const kAudioUnitErr_MissingKey: OSStatus = -66741;
pub const kAudioComponentErr_DuplicateDescription: OSStatus = -66752;
pub const kAudioComponentErr_UnsupportedType: OSStatus = -66751;
pub const kAudioComponentErr_TooManyInstances: OSStatus = -66750;
pub const kAudioComponentErr_NotPermitted: OSStatus = -66748;
pub const kAudioComponentErr_InitializationTimedOut: OSStatus = -66747;
pub const kAudioComponentErr_InvalidFormat: OSStatus = -66746;

pub type AUParameterEventType = u32;

pub const kParameterEvent_Immediate: AUParameterEventType = 1;
pub const kParameterEvent_Ramped: AUParameterEventType = 2;

pub type AudioUnitPropertyID = u32;
pub type AudioUnitScope = u32;
pub type AudioUnitElement = u32;
pub type AudioUnitParameterID = u32;
pub type AudioUnitParameterValue = f32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitParameterEventRamp {
    pub startBufferOffset: i32,
    pub durationInFrames: u32,
    pub startValue: AudioUnitParameterValue,
    pub endValue: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitParameterEventImmediate {
    pub bufferOffset: u32,
    pub value: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union AudioUnitParameterEventValues {
    pub ramp: AudioUnitParameterEventRamp,
    pub immediate: AudioUnitParameterEventImmediate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitParameterEvent {
    pub scope: AudioUnitScope,
    pub element: AudioUnitElement,
    pub parameter: AudioUnitParameterID,
    pub eventType: AUParameterEventType,
    pub eventValues: AudioUnitParameterEventValues,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitParameter {
    pub mAudioUnit: AudioUnit,
    pub mParameterID: AudioUnitParameterID,
    pub mScope: AudioUnitScope,
    pub mElement: AudioUnitElement,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitProperty {
    pub mAudioUnit: AudioUnit,
    pub mPropertyID: AudioUnitPropertyID,
    pub mScope: AudioUnitScope,
    pub mElement: AudioUnitElement,
}

pub type AURenderCallback = unsafe extern "C" fn(
    inRefCon: *mut c_void,
    ioActionFlags: *const AudioUnitRenderActionFlags,
    inTimeStamp: *const AudioTimeStamp,
    inBusNumber: u32,
    inNumberFrames: u32,
    ioData: *mut AudioBufferList,
) -> OSStatus;

pub type AudioUnitPropertyListenerProc = unsafe extern "C" fn(
    inRefCon: *mut c_void,
    inUnit: AudioUnit,
    inID: AudioUnitPropertyID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
);

pub type AUInputSamplesInOutputCallback = unsafe extern "C" fn(
    inRefCon: *mut c_void,
    inOutputTimeStamp: *const AudioTimeStamp,
    inInputSample: f64,
    inNumberInputSamples: f64,
);

#[cfg_attr(target_os = "macos", link(name = "AudioToolbox", kind = "framework"))]
unsafe extern "C" {
    pub static kAudioComponentRegistrationsChangedNotification: CFStringRef;
    pub static kAudioComponentInstanceInvalidationNotification: CFStringRef;
}

#[cfg_attr(target_os = "macos", link(name = "AudioToolbox", kind = "framework"))]
unsafe extern "C" {
    pub fn AudioUnitInitialize(inUnit: AudioUnit) -> OSStatus;

    pub fn AudioUnitUninitialize(inUnit: AudioUnit) -> OSStatus;

    pub fn AudioUnitGetPropertyInfo(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        outDataSize: *mut u32,
        outWritable: *mut Boolean,
    ) -> OSStatus;

    pub fn AudioUnitGetProperty(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        outData: *mut c_void,
        ioDataSize: *mut u32,
    ) -> OSStatus;

    pub fn AudioUnitSetProperty(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        inData: *const c_void,
        inDataSize: u32,
    ) -> OSStatus;

    pub fn AudioUnitAddPropertyListener(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inProc: Option<AudioUnitPropertyListenerProc>,
        inProcUserData: *mut c_void,
    ) -> OSStatus;

    pub fn AudioUnitRemovePropertyListenerWithUserData(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inProc: Option<AudioUnitPropertyListenerProc>,
        inProcUserData: *mut c_void,
    ) -> OSStatus;

    pub fn AudioUnitAddRenderNotify(
        inUnit: AudioUnit,
        inProc: Option<AURenderCallback>,
        inProcUserData: *mut c_void,
    ) -> OSStatus;

    pub fn AudioUnitRemoveRenderNotify(
        inUnit: AudioUnit,
        inProc: Option<AURenderCallback>,
        inProcUserData: *mut c_void,
    ) -> OSStatus;

    pub fn AudioUnitGetParameter(
        inUnit: AudioUnit,
        inID: AudioUnitParameterID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        outValue: *mut AudioUnitParameterValue,
    ) -> OSStatus;

    pub fn AudioUnitSetParameter(
        inUnit: AudioUnit,
        inID: AudioUnitParameterID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        inValue: AudioUnitParameterValue,
        inBufferOffsetInFrames: u32,
    ) -> OSStatus;

    pub fn AudioUnitScheduleParameters(
        inUnit: AudioUnit,
        inParameterEvent: *const AudioUnitParameterEvent,
        inNumParamEvents: u32,
    ) -> OSStatus;

    pub fn AudioUnitRender(
        inUnit: AudioUnit,
        ioActionFlags: *const AudioUnitRenderActionFlags,
        inTimeStamp: *const AudioTimeStamp,
        inOutputBusNumber: u32,
        inNumberFrames: u32,
        ioData: *mut AudioBufferList,
    ) -> OSStatus;

    pub fn AudioUnitProcess(
        inUnit: AudioUnit,
        ioActionFlags: *const AudioUnitRenderActionFlags,
        inTimeStamp: *const AudioTimeStamp,
        inNumberFrames: u32,
        ioData: *mut AudioBufferList,
    ) -> OSStatus;

    pub fn AudioUnitProcessMultiple(
        inUnit: AudioUnit,
        ioActionFlags: *const AudioUnitRenderActionFlags,
        inTimeStamp: *const AudioTimeStamp,
        inNumberFrames: u32,
        inNumberInputBufferLists: u32,
        inInputBufferLists: *const *const AudioBufferList,
        inNumberOutputBufferLists: u32,
        ioOutputBufferLists: *mut *mut AudioBufferList,
    ) -> OSStatus;

    pub fn AudioUnitReset(
        inUnit: AudioUnit,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
    ) -> OSStatus;

    pub fn AudioOutputUnitPublish(
        inDesc: *const AudioComponentDescription,
        inName: CFStringRef,
        inVersion: u32,
        inOutputUnit: AudioUnit,
    ) -> OSStatus;
}

pub const kAudioUnitRange: i16 = 0x0000;
pub const kAudioUnitInitializeSelect: i16 = 0x0001;
pub const kAudioUnitUninitializeSelect: i16 = 0x0002;
pub const kAudioUnitGetPropertyInfoSelect: i16 = 0x0003;
pub const kAudioUnitGetPropertySelect: i16 = 0x0004;
pub const kAudioUnitSetPropertySelect: i16 = 0x0005;
pub const kAudioUnitAddPropertyListenerSelect: i16 = 0x000A;
pub const kAudioUnitRemovePropertyListenerSelect: i16 = 0x000B;
pub const kAudioUnitRemovePropertyListenerWithUserDataSelect: i16 = 0x0012;
pub const kAudioUnitAddRenderNotifySelect: i16 = 0x000F;
pub const kAudioUnitRemoveRenderNotifySelect: i16 = 0x0010;
pub const kAudioUnitGetParameterSelect: i16 = 0x0006;
pub const kAudioUnitSetParameterSelect: i16 = 0x0007;
pub const kAudioUnitScheduleParametersSelect: i16 = 0x0011;
pub const kAudioUnitRenderSelect: i16 = 0x000E;
pub const kAudioUnitResetSelect: i16 = 0x0009;
pub const kAudioUnitComplexRenderSelect: i16 = 0x0013;
pub const kAudioUnitProcessSelect: i16 = 0x0014;
pub const kAudioUnitProcessMultipleSelect: i16 = 0x0015;

pub type AudioUnitInitializeProc = unsafe extern "C" fn(self_: *mut c_void) -> OSStatus;

pub type AudioUnitUninitializeProc = unsafe extern "C" fn(self_: *mut c_void) -> OSStatus;

pub type AudioUnitGetPropertyInfoProc = unsafe extern "C" fn(
    self_: *mut c_void,
    prop: AudioUnitPropertyID,
    scope: AudioUnitScope,
    elem: AudioUnitElement,
    outDataSize: *mut u32,
    outWritable: *mut Boolean,
) -> OSStatus;

pub type AudioUnitGetPropertyProc = unsafe extern "C" fn(
    self_: *mut c_void,
    inID: AudioUnitPropertyID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
    outData: *mut c_void,
    ioDataSize: *mut u32,
) -> OSStatus;

pub type AudioUnitSetPropertyProc = unsafe extern "C" fn(
    self_: *mut c_void,
    inID: AudioUnitPropertyID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
    inData: *const c_void,
    inDataSize: u32,
) -> OSStatus;

pub type AudioUnitAddPropertyListenerProc = unsafe extern "C" fn(
    self_: *mut c_void,
    prop: AudioUnitPropertyID,
    proc: Option<AudioUnitPropertyListenerProc>,
    userData: *mut c_void,
) -> OSStatus;

pub type AudioUnitRemovePropertyListenerProc = unsafe extern "C" fn(
    self_: *mut c_void,
    prop: AudioUnitPropertyID,
    proc: Option<AudioUnitPropertyListenerProc>,
) -> OSStatus;

pub type AudioUnitRemovePropertyListenerWithUserDataProc = unsafe extern "C" fn(
    self_: *mut c_void,
    prop: AudioUnitPropertyID,
    proc: Option<AudioUnitPropertyListenerProc>,
    userData: *mut c_void,
) -> OSStatus;

pub type AudioUnitAddRenderNotifyProc = unsafe extern "C" fn(
    self_: *mut c_void,
    proc: Option<AURenderCallback>,
    userData: *mut c_void,
) -> OSStatus;

pub type AudioUnitRemoveRenderNotifyProc = unsafe extern "C" fn(
    self_: *mut c_void,
    proc: Option<AURenderCallback>,
    userData: *mut c_void,
) -> OSStatus;

pub type AudioUnitScheduleParametersProc = unsafe extern "C" fn(
    self_: *mut c_void,
    events: *const AudioUnitParameterEvent,
    numEvents: u32,
) -> OSStatus;

pub type AudioUnitResetProc = unsafe extern "C" fn(
    self_: *mut c_void,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
) -> OSStatus;

pub type AudioUnitComplexRenderProc = unsafe extern "C" fn(
    self_: *mut c_void,
    ioActionFlags: *const AudioUnitRenderActionFlags,
    nTimeStamp: *const AudioTimeStamp,
    inOutputBusNumber: u32,
    inNumberOfPackets: u32,
    outNumberOfPackets: *mut u32,
    outPacketDescriptions: *mut AudioStreamPacketDescription,
    ioData: *mut AudioBufferList,
    outMetadata: *mut c_void,
    outMetadataByteSize: *mut u32,
) -> OSStatus;

pub type AudioUnitProcessProc = unsafe extern "C" fn(
    self_: *mut c_void,
    ioActionFlags: *const AudioUnitRenderActionFlags,
    nTimeStamp: *const AudioTimeStamp,
    inNumberFrames: u32,
    ioData: *mut AudioBufferList,
) -> OSStatus;

pub type AudioUnitProcessMultipleProc = unsafe extern "C" fn(
    self_: *mut c_void,
    ioActionFlags: *const AudioUnitRenderActionFlags,
    nTimeStamp: *const AudioTimeStamp,
    inNumberFrames: u32,
    inNumberInputBufferLists: u32,
    inInputBufferLists: *const *const AudioBufferList,
    inNumberOutputBufferLists: u32,
    ioOutputBufferLists: *mut *mut AudioBufferList,
) -> OSStatus;

pub type AudioUnitGetParameterProc = unsafe extern "C" fn(
    inComponentStorage: *mut c_void,
    inID: AudioUnitParameterID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
    outValue: *mut AudioUnitParameterValue,
) -> OSStatus;

pub type AudioUnitSetParameterProc = unsafe extern "C" fn(
    inComponentStorage: *mut c_void,
    inID: AudioUnitParameterID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
    inValue: AudioUnitParameterValue,
    inBufferOffsetInFrames: u32,
) -> OSStatus;

pub type AudioUnitRenderProc = unsafe extern "C" fn(
    inComponentStorage: *mut c_void,
    ioActionFlags: *const AudioUnitRenderActionFlags,
    inTimeStamp: *const AudioTimeStamp,
    inOutputBusNumber: u32,
    inNumberFrames: u32,
    ioData: *mut AudioBufferList,
) -> OSStatus;

// AudioOutputUnit.h

#[cfg_attr(target_os = "macos", link(name = "AudioToolbox", kind = "framework"))]
unsafe extern "C" {
    pub fn AudioOutputUnitStart(ci: AudioUnit) -> OSStatus;
    pub fn AudioOutputUnitStop(ci: AudioUnit) -> OSStatus;
}

pub const kAudioOutputUnitRange: u32 = 0x0200;
pub const kAudioOutputUnitStartSelect: u32 = 0x0201;
pub const kAudioOutputUnitStopSelect: u32 = 0x0202;

pub type AudioOutputUnitStartProc = unsafe extern "C" fn(self_: *mut c_void) -> OSStatus;
pub type AudioOutputUnitStopProc = unsafe extern "C" fn(self_: *mut c_void) -> OSStatus;

// AudioUnitProperties.h

pub const kAudioUnitScope_Global: AudioUnitScope = 0;
pub const kAudioUnitScope_Input: AudioUnitScope = 1;
pub const kAudioUnitScope_Output: AudioUnitScope = 2;
pub const kAudioUnitScope_Group: AudioUnitScope = 3;
pub const kAudioUnitScope_Part: AudioUnitScope = 4;
pub const kAudioUnitScope_Note: AudioUnitScope = 5;
pub const kAudioUnitScope_Layer: AudioUnitScope = 6;
pub const kAudioUnitScope_LayerItem: AudioUnitScope = 7;

pub const kAudioUnitProperty_ClassInfo: AudioUnitPropertyID = 0;
pub const kAudioUnitProperty_MakeConnection: AudioUnitPropertyID = 1;
pub const kAudioUnitProperty_SampleRate: AudioUnitPropertyID = 2;
pub const kAudioUnitProperty_ParameterList: AudioUnitPropertyID = 3;
pub const kAudioUnitProperty_ParameterInfo: AudioUnitPropertyID = 4;
pub const kAudioUnitProperty_CPULoad: AudioUnitPropertyID = 6;
pub const kAudioUnitProperty_StreamFormat: AudioUnitPropertyID = 8;
pub const kAudioUnitProperty_ElementCount: AudioUnitPropertyID = 11;
pub const kAudioUnitProperty_Latency: AudioUnitPropertyID = 12;
pub const kAudioUnitProperty_SupportedNumChannels: AudioUnitPropertyID = 13;
pub const kAudioUnitProperty_MaximumFramesPerSlice: AudioUnitPropertyID = 14;
pub const kAudioUnitProperty_ParameterValueStrings: AudioUnitPropertyID = 16;
pub const kAudioUnitProperty_AudioChannelLayout: AudioUnitPropertyID = 19;
pub const kAudioUnitProperty_TailTime: AudioUnitPropertyID = 20;
pub const kAudioUnitProperty_BypassEffect: AudioUnitPropertyID = 21;
pub const kAudioUnitProperty_LastRenderError: AudioUnitPropertyID = 22;
pub const kAudioUnitProperty_SetRenderCallback: AudioUnitPropertyID = 23;
pub const kAudioUnitProperty_FactoryPresets: AudioUnitPropertyID = 24;
pub const kAudioUnitProperty_RenderQuality: AudioUnitPropertyID = 26;
pub const kAudioUnitProperty_HostCallbacks: AudioUnitPropertyID = 27;
pub const kAudioUnitProperty_InPlaceProcessing: AudioUnitPropertyID = 29;
pub const kAudioUnitProperty_ElementName: AudioUnitPropertyID = 30;
pub const kAudioUnitProperty_SupportedChannelLayoutTags: AudioUnitPropertyID = 32;
pub const kAudioUnitProperty_PresentPreset: AudioUnitPropertyID = 36;
pub const kAudioUnitProperty_DependentParameters: AudioUnitPropertyID = 45;
pub const kAudioUnitProperty_InputSamplesInOutput: AudioUnitPropertyID = 49;
pub const kAudioUnitProperty_ShouldAllocateBuffer: AudioUnitPropertyID = 51;
pub const kAudioUnitProperty_FrequencyResponse: AudioUnitPropertyID = 52;
pub const kAudioUnitProperty_ParameterHistoryInfo: AudioUnitPropertyID = 53;
pub const kAudioUnitProperty_NickName: AudioUnitPropertyID = 54;
pub const kAudioUnitProperty_OfflineRender: AudioUnitPropertyID = 37;
pub const kAudioUnitProperty_ParameterIDName: AudioUnitPropertyID = 34;
pub const kAudioUnitProperty_ParameterStringFromValue: AudioUnitPropertyID = 33;
pub const kAudioUnitProperty_ParameterClumpName: AudioUnitPropertyID = 35;
pub const kAudioUnitProperty_ParameterValueFromString: AudioUnitPropertyID = 38;
pub const kAudioUnitProperty_ContextName: AudioUnitPropertyID = 25;
pub const kAudioUnitProperty_PresentationLatency: AudioUnitPropertyID = 40;
pub const kAudioUnitProperty_ClassInfoFromDocument: AudioUnitPropertyID = 50;
pub const kAudioUnitProperty_RequestViewController: AudioUnitPropertyID = 56;
pub const kAudioUnitProperty_ParametersForOverview: AudioUnitPropertyID = 57;
pub const kAudioUnitProperty_SupportsMPE: AudioUnitPropertyID = 58;
pub const kAudioUnitProperty_RenderContextObserver: AudioUnitPropertyID = 60;
pub const kAudioUnitProperty_LoadedOutOfProcess: AudioUnitPropertyID = 62;
pub const kAudioUnitProperty_FastDispatch: AudioUnitPropertyID = 5;
pub const kAudioUnitProperty_SetExternalBuffer: AudioUnitPropertyID = 15;
pub const kAudioUnitProperty_GetUIComponentList: AudioUnitPropertyID = 18;
pub const kAudioUnitProperty_CocoaUI: AudioUnitPropertyID = 31;
pub const kAudioUnitProperty_IconLocation: AudioUnitPropertyID = 39;
pub const kAudioUnitProperty_AUHostIdentifier: AudioUnitPropertyID = 46;
pub const kAudioUnitProperty_MIDIOutputCallbackInfo: AudioUnitPropertyID = 47;
pub const kAudioUnitProperty_MIDIOutputCallback: AudioUnitPropertyID = 48;
pub const kAudioUnitProperty_RemoteControlEventListener: AudioUnitPropertyID = 100;
pub const kAudioUnitProperty_IsInterAppConnected: AudioUnitPropertyID = 101;
pub const kAudioUnitProperty_PeerURL: AudioUnitPropertyID = 102;

pub const kAUPresetVersionKey: &str = "version";
pub const kAUPresetTypeKey: &str = "type";
pub const kAUPresetSubtypeKey: &str = "subtype";
pub const kAUPresetManufacturerKey: &str = "manufacturer";
pub const kAUPresetDataKey: &str = "data";
pub const kAUPresetNameKey: &str = "name";
pub const kAUPresetNumberKey: &str = "preset-number";
pub const kAUPresetRenderQualityKey: &str = "render-quality";
pub const kAUPresetCPULoadKey: &str = "cpu-load";
pub const kAUPresetElementNameKey: &str = "element-name";
pub const kAUPresetExternalFileRefs: &str = "file-references";
pub const kAUPresetVSTDataKey: &str = "vstdata";
pub const kAUPresetVSTPresetKey: &str = "vstpreset";
pub const kAUPresetMASDataKey: &str = "masdata";
pub const kAUPresetPartKey: &str = "part";

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitConnection {
    pub sourceAudioUnit: AudioUnit,
    pub sourceOutputNumber: u32,
    pub destInputNumber: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AUChannelInfo {
    pub inChannels: i16,
    pub outChannels: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitExternalBuffer {
    pub buffer: *mut u8,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AURenderCallbackStruct {
    pub inputProc: Option<AURenderCallback>,
    pub inputProcRefCon: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AUPreset {
    pub presetNumber: i32,
    pub presetName: CFStringRef,
}

pub const kRenderQuality_Max: u32 = 127;
pub const kRenderQuality_High: u32 = 96;
pub const kRenderQuality_Medium: u32 = 64;
pub const kRenderQuality_Low: u32 = 32;
pub const kRenderQuality_Min: u32 = 0;

pub const kNumberOfResponseFrequencies: u32 = 1024;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitFrequencyResponseBin {
    pub mFrequency: f64,
    pub mMagnitude: f64,
}

pub type HostCallback_GetBeatAndTempo = unsafe extern "C" fn(
    inHostUserData: *mut c_void,
    outCurrentBeat: *mut f64,
    outCurrentTempo: *mut f64,
) -> OSStatus;

pub type HostCallback_GetMusicalTimeLocation = unsafe extern "C" fn(
    inHostUserData: *mut c_void,
    outDeltaSampleOffsetToNextBeat: *mut u32,
    outTimeSig_Numerator: *mut f32,
    outTimeSig_Denominator: *mut u32,
    outCurrentMeasureDownBeat: *mut f64,
) -> OSStatus;

pub type HostCallback_GetTransportState = unsafe extern "C" fn(
    inHostUserData: *mut c_void,
    outIsPlaying: *mut Boolean,
    outTransportStateChanged: *mut Boolean,
    outCurrentSampleInTimeLine: *mut f64,
    outIsCycling: *mut Boolean,
    outCycleStartBeat: *mut f64,
    outCycleEndBeat: *mut f64,
) -> OSStatus;

pub type HostCallback_GetTransportState2 = unsafe extern "C" fn(
    inHostUserData: *mut c_void,
    outIsPlaying: *mut Boolean,
    outIsRecording: *mut Boolean,
    outTransportStateChanged: *mut Boolean,
    outCurrentSampleInTimeLine: *mut f64,
    outIsCycling: *mut Boolean,
    outCycleStartBeat: *mut f64,
    outCycleEndBeat: *mut f64,
) -> OSStatus;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HostCallbackInfo {
    pub hostUserData: *mut c_void,
    pub beatAndTempoProc: Option<HostCallback_GetBeatAndTempo>,
    pub musicalTimeLocationProc: Option<HostCallback_GetMusicalTimeLocation>,
    pub transportStateProc: Option<HostCallback_GetTransportState>,
    pub transportStateProc2: Option<HostCallback_GetTransportState2>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AUDependentParameter {
    pub mScope: AudioUnitScope,
    pub mParameterID: AudioUnitParameterID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitCocoaViewInfo {
    pub mCocoaAUViewBundleLocation: CFURLRef,
    pub mCocoaAUViewClass: [CFStringRef; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AUHostVersionIdentifier {
    pub hostName: CFStringRef,
    pub hostVersion: u32,
}

pub type AUMIDIOutputCallback = unsafe extern "C" fn(
    userData: *mut c_void,
    timeStamp: *const AudioTimeStamp,
    midiOutNum: u32,
    pktlist: *const MIDIPacketList,
) -> OSStatus;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AUMIDIOutputCallbackStruct {
    pub midiOutputCallback: Option<AUMIDIOutputCallback>,
    pub userData: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AUInputSamplesInOutputCallbackStruct {
    pub inputToOutputCallback: Option<AUInputSamplesInOutputCallback>,
    pub userData: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitParameterHistoryInfo {
    pub updatesPerSecond: f32,
    pub historyDurationInSeconds: f32,
}

pub type AudioUnitParameterUnit = u32;

pub const kAudioUnitParameterUnit_Generic: AudioUnitParameterUnit = 0;
pub const kAudioUnitParameterUnit_Indexed: AudioUnitParameterUnit = 1;
pub const kAudioUnitParameterUnit_Boolean: AudioUnitParameterUnit = 2;
pub const kAudioUnitParameterUnit_Percent: AudioUnitParameterUnit = 3;
pub const kAudioUnitParameterUnit_Seconds: AudioUnitParameterUnit = 4;
pub const kAudioUnitParameterUnit_SampleFrames: AudioUnitParameterUnit = 5;
pub const kAudioUnitParameterUnit_Phase: AudioUnitParameterUnit = 6;
pub const kAudioUnitParameterUnit_Rate: AudioUnitParameterUnit = 7;
pub const kAudioUnitParameterUnit_Hertz: AudioUnitParameterUnit = 8;
pub const kAudioUnitParameterUnit_Cents: AudioUnitParameterUnit = 9;
pub const kAudioUnitParameterUnit_RelativeSemiTones: AudioUnitParameterUnit = 10;
pub const kAudioUnitParameterUnit_MIDINoteNumber: AudioUnitParameterUnit = 11;
pub const kAudioUnitParameterUnit_MIDIController: AudioUnitParameterUnit = 12;
pub const kAudioUnitParameterUnit_Decibels: AudioUnitParameterUnit = 13;
pub const kAudioUnitParameterUnit_LinearGain: AudioUnitParameterUnit = 14;
pub const kAudioUnitParameterUnit_Degrees: AudioUnitParameterUnit = 15;
pub const kAudioUnitParameterUnit_EqualPowerCrossfade: AudioUnitParameterUnit = 16;
pub const kAudioUnitParameterUnit_MixerFaderCurve1: AudioUnitParameterUnit = 17;
pub const kAudioUnitParameterUnit_Pan: AudioUnitParameterUnit = 18;
pub const kAudioUnitParameterUnit_Meters: AudioUnitParameterUnit = 19;
pub const kAudioUnitParameterUnit_AbsoluteCents: AudioUnitParameterUnit = 20;
pub const kAudioUnitParameterUnit_Octaves: AudioUnitParameterUnit = 21;
pub const kAudioUnitParameterUnit_BPM: AudioUnitParameterUnit = 22;
pub const kAudioUnitParameterUnit_Beats: AudioUnitParameterUnit = 23;
pub const kAudioUnitParameterUnit_Milliseconds: AudioUnitParameterUnit = 24;
pub const kAudioUnitParameterUnit_Ratio: AudioUnitParameterUnit = 25;
pub const kAudioUnitParameterUnit_CustomUnit: AudioUnitParameterUnit = 26;

pub type AudioUnitParameterOptions = u32;

pub const kAudioUnitParameterFlag_CFNameRelease: AudioUnitParameterOptions = 1 << 4;
pub const kAudioUnitParameterFlag_OmitFromPresets: AudioUnitParameterOptions = 1 << 13;
pub const kAudioUnitParameterFlag_PlotHistory: AudioUnitParameterOptions = 1 << 14;
pub const kAudioUnitParameterFlag_MeterReadOnly: AudioUnitParameterOptions = 1 << 15;
pub const kAudioUnitParameterFlag_DisplayMask: AudioUnitParameterOptions = (7 << 16) | (1 << 22);
pub const kAudioUnitParameterFlag_DisplaySquareRoot: AudioUnitParameterOptions = 1 << 16;
pub const kAudioUnitParameterFlag_DisplaySquared: AudioUnitParameterOptions = 2 << 16;
pub const kAudioUnitParameterFlag_DisplayCubed: AudioUnitParameterOptions = 3 << 16;
pub const kAudioUnitParameterFlag_DisplayCubeRoot: AudioUnitParameterOptions = 4 << 16;
pub const kAudioUnitParameterFlag_DisplayExponential: AudioUnitParameterOptions = 5 << 16;
pub const kAudioUnitParameterFlag_HasClump: AudioUnitParameterOptions = 1 << 20;
pub const kAudioUnitParameterFlag_ValuesHaveStrings: AudioUnitParameterOptions = 1 << 21;
pub const kAudioUnitParameterFlag_DisplayLogarithmic: AudioUnitParameterOptions = 1 << 22;
pub const kAudioUnitParameterFlag_IsHighResolution: AudioUnitParameterOptions = 1 << 23;
pub const kAudioUnitParameterFlag_NonRealTime: AudioUnitParameterOptions = 1 << 24;
pub const kAudioUnitParameterFlag_CanRamp: AudioUnitParameterOptions = 1 << 25;
pub const kAudioUnitParameterFlag_ExpertMode: AudioUnitParameterOptions = 1 << 26;
pub const kAudioUnitParameterFlag_HasCFNameString: AudioUnitParameterOptions = 1 << 27;
pub const kAudioUnitParameterFlag_IsGlobalMeta: AudioUnitParameterOptions = 1 << 28;
pub const kAudioUnitParameterFlag_IsElementMeta: AudioUnitParameterOptions = 1 << 29;
pub const kAudioUnitParameterFlag_IsReadable: AudioUnitParameterOptions = 1 << 30;
pub const kAudioUnitParameterFlag_IsWritable: AudioUnitParameterOptions = 1 << 31;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitParameterInfo {
    pub name: [c_char; 52],
    pub unitName: CFStringRef,
    pub clumpID: u32,
    pub cfNameString: CFStringRef,
    pub unit: AudioUnitParameterUnit,
    pub minValue: AudioUnitParameterValue,
    pub maxValue: AudioUnitParameterValue,
    pub defaultValue: AudioUnitParameterValue,
    pub flags: AudioUnitParameterOptions,
}

pub const kAudioUnitClumpID_System: u32 = 0;

#[inline]
pub const fn GetAudioUnitParameterDisplayType(
    flags: AudioUnitParameterOptions,
) -> AudioUnitParameterOptions {
    flags & kAudioUnitParameterFlag_DisplayMask
}

#[inline]
pub const fn SetAudioUnitParameterDisplayType(
    flags: AudioUnitParameterOptions,
    displayType: AudioUnitParameterOptions,
) -> AudioUnitParameterOptions {
    (flags & !kAudioUnitParameterFlag_DisplayMask) | displayType
}

pub const kAudioUnitParameterName_Full: u32 = u32::MAX;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitParameterNameInfo {
    pub inID: AudioUnitParameterID,
    pub inDesiredLength: i32,
    pub outName: CFStringRef,
}

pub type AudioUnitParameterIDName = AudioUnitParameterNameInfo;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitParameterStringFromValue {
    pub inParamIDL: AudioUnitParameterID,
    pub inValueL: *const AudioUnitParameterValue,
    pub outStringL: CFStringRef,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitParameterValueFromString {
    pub inParamIDL: AudioUnitParameterID,
    pub inStringL: CFStringRef,
    pub outValueL: AudioUnitParameterValue,
}

pub type AudioUnitRemoteControlEvent = u32;

pub const kAudioUnitRemoteControlEvent_TogglePlayPause: AudioUnitRemoteControlEvent = 1;
pub const kAudioUnitRemoteControlEvent_ToggleRecord: AudioUnitRemoteControlEvent = 2;
pub const kAudioUnitRemoteControlEvent_Rewind: AudioUnitRemoteControlEvent = 3;

pub const kAudioUnitConfigurationInfo_HasCustomView: &str = "HasCustomView";
pub const kAudioUnitConfigurationInfo_ChannelConfigurations: &str = "ChannelConfigurations";
pub const kAudioUnitConfigurationInfo_InitialInputs: &str = "InitialInputs";
pub const kAudioUnitConfigurationInfo_InitialOutputs: &str = "InitialOutputs";
pub const kAudioUnitConfigurationInfo_IconURL: &str = "IconURL";
pub const kAudioUnitConfigurationInfo_BusCountWritable: &str = "BusCountWritable";
pub const kAudioUnitConfigurationInfo_SupportedChannelLayoutTags: &str =
    "SupportedChannelLayoutTags";

pub const kAudioUnitProperty_AllParameterMIDIMappings: AudioUnitPropertyID = 41;
pub const kAudioUnitProperty_AddParameterMIDIMapping: AudioUnitPropertyID = 42;
pub const kAudioUnitProperty_RemoveParameterMIDIMapping: AudioUnitPropertyID = 43;
pub const kAudioUnitProperty_HotMapParameterMIDIMapping: AudioUnitPropertyID = 44;

pub type AUParameterMIDIMappingFlags = u32;

pub const kAUParameterMIDIMapping_AnyChannelFlag: AUParameterMIDIMappingFlags = 1 << 0;
pub const kAUParameterMIDIMapping_AnyNoteFlag: AUParameterMIDIMappingFlags = 1 << 1;
pub const kAUParameterMIDIMapping_SubRange: AUParameterMIDIMappingFlags = 1 << 2;
pub const kAUParameterMIDIMapping_Toggle: AUParameterMIDIMappingFlags = 1 << 3;
pub const kAUParameterMIDIMapping_Bipolar: AUParameterMIDIMappingFlags = 1 << 4;
pub const kAUParameterMIDIMapping_Bipolar_On: AUParameterMIDIMappingFlags = 1 << 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AUParameterMIDIMapping {
    pub mScope: AudioUnitScope,
    pub mElement: AudioUnitElement,
    pub mParameterID: AudioUnitParameterID,
    pub mFlags: AUParameterMIDIMappingFlags,
    pub mSubRangeMin: AudioUnitParameterValue,
    pub mSubRangeMax: AudioUnitParameterValue,
    pub mStatus: u8,
    pub mData1: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub reserved3: u32,
}

pub const kMusicDeviceProperty_MIDIXMLNames: AudioUnitPropertyID = 1006;
pub const kMusicDeviceProperty_PartGroup: AudioUnitPropertyID = 1010;
pub const kMusicDeviceProperty_DualSchedulingMode: AudioUnitPropertyID = 1013;
pub const kMusicDeviceProperty_SupportsStartStopNote: AudioUnitPropertyID = 1014;

pub const kMusicDeviceSampleFrameMask_SampleOffset: u32 = 0xFFFFFF;
pub const kMusicDeviceSampleFrameMask_IsScheduled: u32 = 0x01000000;

pub const kAudioUnitOfflineProperty_InputSize: AudioUnitPropertyID = 3020;
pub const kAudioUnitOfflineProperty_OutputSize: AudioUnitPropertyID = 3021;
pub const kAudioUnitOfflineProperty_StartOffset: AudioUnitPropertyID = 3022;
pub const kAudioUnitOfflineProperty_PreflightRequirements: AudioUnitPropertyID = 3023;
pub const kAudioUnitOfflineProperty_PreflightName: AudioUnitPropertyID = 3024;

pub const kOfflinePreflight_NotRequired: u32 = 0;
pub const kOfflinePreflight_Optional: u32 = 1;
pub const kOfflinePreflight_Required: u32 = 2;

pub const kAudioUnitMigrateProperty_FromPlugin: u32 = 4000;
pub const kAudioUnitMigrateProperty_OldAutomation: u32 = 4001;

pub const kOtherPluginFormat_Undefined: u32 = 0;
pub const kOtherPluginFormat_kMAS: u32 = 1;
pub const kOtherPluginFormat_kVST: u32 = 2;
pub const kOtherPluginFormat_AU: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitOtherPluginDesc {
    pub format: u32,
    pub plugin: AudioClassDescription,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitParameterValueTranslation {
    pub otherDesc: AudioUnitOtherPluginDesc,
    pub otherParamID: u32,
    pub otherValue: f32,
    pub auParamID: AudioUnitParameterID,
    pub auValue: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitPresetMAS_SettingData {
    pub isStockSetting: u32,
    pub settingID: u32,
    pub dataLen: u32,
    pub data: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitPresetMAS_Settings {
    pub manufacturerID: u32,
    pub effectID: u32,
    pub variantID: u32,
    pub settingsVersion: u32,
    pub numberOfSettings: u32,
    pub settings: [AudioUnitPresetMAS_SettingData; 0],
}

pub const kAudioUnitProperty_SampleRateConverterComplexity: AudioUnitPropertyID = 3014;

pub const kAudioUnitSampleRateConverterComplexity_Linear: u32 = u32::from_be_bytes(*b"line");
pub const kAudioUnitSampleRateConverterComplexity_Normal: u32 = u32::from_be_bytes(*b"norm");
pub const kAudioUnitSampleRateConverterComplexity_Mastering: u32 = u32::from_be_bytes(*b"bats");

pub const kAudioOutputUnitProperty_CurrentDevice: AudioUnitPropertyID = 2000;
pub const kAudioOutputUnitProperty_IsRunning: AudioUnitPropertyID = 2001;
pub const kAudioOutputUnitProperty_ChannelMap: AudioUnitPropertyID = 2002;
pub const kAudioOutputUnitProperty_EnableIO: AudioUnitPropertyID = 2003;
pub const kAudioOutputUnitProperty_StartTime: AudioUnitPropertyID = 2004;
pub const kAudioOutputUnitProperty_SetInputCallback: AudioUnitPropertyID = 2005;
pub const kAudioOutputUnitProperty_HasIO: AudioUnitPropertyID = 2006;
pub const kAudioOutputUnitProperty_StartTimestampsAtZero: AudioUnitPropertyID = 2007;

pub const kAudioOutputUnitProperty_OSWorkgroup: AudioUnitPropertyID = 2015;

pub const kAudioOutputUnitProperty_MIDICallbacks: AudioUnitPropertyID = 2010;
pub const kAudioOutputUnitProperty_HostReceivesRemoteControlEvents: AudioUnitPropertyID = 2011;
pub const kAudioOutputUnitProperty_RemoteControlToHost: AudioUnitPropertyID = 2012;
pub const kAudioOutputUnitProperty_HostTransportState: AudioUnitPropertyID = 2013;
pub const kAudioOutputUnitProperty_NodeComponentDescription: AudioUnitPropertyID = 2014;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioOutputUnitMIDICallbacks {
    pub userData: *mut c_void,
    pub MIDIEventProc: Option<
        unsafe extern "C" fn(
            userData: *mut c_void,
            inStatus: u32,
            inData1: u32,
            inData2: u32,
            inOffsetSampleFrame: u32,
        ),
    >,
    pub MIDISysExProc:
        Option<unsafe extern "C" fn(userData: *mut c_void, inData: *const u8, inLength: u32)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioOutputUnitStartAtTimeParams {
    pub mTimestamp: AudioTimeStamp,
    pub mFlags: u32,
}

pub const kAUVoiceIOProperty_BypassVoiceProcessing: AudioUnitPropertyID = 2100;
pub const kAUVoiceIOProperty_VoiceProcessingEnableAGC: AudioUnitPropertyID = 2101;
pub const kAUVoiceIOProperty_MuteOutput: AudioUnitPropertyID = 2104;

pub const kAUNBandEQProperty_NumberOfBands: AudioUnitPropertyID = 2200;
pub const kAUNBandEQProperty_MaxNumberOfBands: AudioUnitPropertyID = 2201;
pub const kAUNBandEQProperty_BiquadCoefficients: AudioUnitPropertyID = 2203;

pub const kAUVoiceIOErr_UnexpectedNumberOfInputChannels: OSStatus = -66784;

pub const kAudioUnitProperty_MeteringMode: AudioUnitPropertyID = 3007;
pub const kAudioUnitProperty_MatrixLevels: AudioUnitPropertyID = 3006;
pub const kAudioUnitProperty_MatrixDimensions: AudioUnitPropertyID = 3009;
pub const kAudioUnitProperty_MeterClipping: AudioUnitPropertyID = 3011;
pub const kAudioUnitProperty_InputAnchorTimeStamp: AudioUnitPropertyID = 3016;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitMeterClipping {
    pub peakValueSinceLastCall: f32,
    pub sawInfinity: Boolean,
    pub sawNotANumber: Boolean,
}

pub const kAudioUnitProperty_ReverbRoomType: AudioUnitPropertyID = 10;
pub const kAudioUnitProperty_UsesInternalReverb: AudioUnitPropertyID = 1005;
pub const kAudioUnitProperty_SpatializationAlgorithm: AudioUnitPropertyID = 3000;
pub const kAudioUnitProperty_SpatialMixerRenderingFlags: AudioUnitPropertyID = 3003;
pub const kAudioUnitProperty_SpatialMixerSourceMode: AudioUnitPropertyID = 3005;
pub const kAudioUnitProperty_SpatialMixerDistanceParams: AudioUnitPropertyID = 3010;
pub const kAudioUnitProperty_SpatialMixerAttenuationCurve: AudioUnitPropertyID = 3013;
pub const kAudioUnitProperty_SpatialMixerOutputType: AudioUnitPropertyID = 3100;
pub const kAudioUnitProperty_SpatialMixerPointSourceInHeadMode: AudioUnitPropertyID = 3103;

pub type AUSpatializationAlgorithm = u32;

pub const kSpatializationAlgorithm_EqualPowerPanning: AUSpatializationAlgorithm = 0;
pub const kSpatializationAlgorithm_SphericalHead: AUSpatializationAlgorithm = 1;
pub const kSpatializationAlgorithm_HRTF: AUSpatializationAlgorithm = 2;
pub const kSpatializationAlgorithm_SoundField: AUSpatializationAlgorithm = 3;
pub const kSpatializationAlgorithm_VectorBasedPanning: AUSpatializationAlgorithm = 4;
pub const kSpatializationAlgorithm_StereoPassThrough: AUSpatializationAlgorithm = 5;
pub const kSpatializationAlgorithm_HRTFHQ: AUSpatializationAlgorithm = 6;
pub const kSpatializationAlgorithm_UseOutputType: AUSpatializationAlgorithm = 7;

pub type AUSpatialMixerSourceMode = u32;

pub const kSpatialMixerSourceMode_SpatializeIfMono: AUSpatialMixerSourceMode = 0;
pub const kSpatialMixerSourceMode_Bypass: AUSpatialMixerSourceMode = 1;
pub const kSpatialMixerSourceMode_PointSource: AUSpatialMixerSourceMode = 2;
pub const kSpatialMixerSourceMode_AmbienceBed: AUSpatialMixerSourceMode = 3;

pub type AUReverbRoomType = u32;

pub const kReverbRoomType_SmallRoom: AUReverbRoomType = 0;
pub const kReverbRoomType_MediumRoom: AUReverbRoomType = 1;
pub const kReverbRoomType_LargeRoom: AUReverbRoomType = 2;
pub const kReverbRoomType_MediumHall: AUReverbRoomType = 3;
pub const kReverbRoomType_LargeHall: AUReverbRoomType = 4;
pub const kReverbRoomType_Plate: AUReverbRoomType = 5;
pub const kReverbRoomType_MediumChamber: AUReverbRoomType = 6;
pub const kReverbRoomType_LargeChamber: AUReverbRoomType = 7;
pub const kReverbRoomType_Cathedral: AUReverbRoomType = 8;
pub const kReverbRoomType_LargeRoom2: AUReverbRoomType = 9;
pub const kReverbRoomType_MediumHall2: AUReverbRoomType = 10;
pub const kReverbRoomType_MediumHall3: AUReverbRoomType = 11;
pub const kReverbRoomType_LargeHall2: AUReverbRoomType = 12;

pub type AUSpatialMixerAttenuationCurve = u32;

pub const kSpatialMixerAttenuationCurve_Power: AUSpatialMixerAttenuationCurve = 0;
pub const kSpatialMixerAttenuationCurve_Exponential: AUSpatialMixerAttenuationCurve = 1;
pub const kSpatialMixerAttenuationCurve_Inverse: AUSpatialMixerAttenuationCurve = 2;
pub const kSpatialMixerAttenuationCurve_Linear: AUSpatialMixerAttenuationCurve = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MixerDistanceParams {
    pub mReferenceDistance: f32,
    pub mMaxDistance: f32,
    pub mMaxAttenuation: f32,
}

pub type AUSpatialMixerRenderingFlags = u32;

pub const kSpatialMixerRenderingFlags_InterAuralDelay: AUSpatialMixerRenderingFlags = 1 << 0;
pub const kSpatialMixerRenderingFlags_DistanceAttenuation: AUSpatialMixerRenderingFlags = 1 << 2;

pub type AUSpatialMixerOutputType = u32;

pub const kSpatialMixerOutputType_Headphones: AUSpatialMixerOutputType = 1;
pub const kSpatialMixerOutputType_BuiltInSpeakers: AUSpatialMixerOutputType = 2;
pub const kSpatialMixerOutputType_ExternalSpeakers: AUSpatialMixerOutputType = 3;

pub type AUSpatialMixerPointSourceInHeadMode = u32;

pub const kSpatialMixerPointSourceInHeadMode_Mono: AUSpatialMixerPointSourceInHeadMode = 0;
pub const kSpatialMixerPointSourceInHeadMode_Bypass: AUSpatialMixerPointSourceInHeadMode = 1;

pub const kAudioUnitProperty_ScheduleAudioSlice: AudioUnitPropertyID = 3300;
pub const kAudioUnitProperty_ScheduleStartTimeStamp: AudioUnitPropertyID = 3301;
pub const kAudioUnitProperty_CurrentPlayTime: AudioUnitPropertyID = 3302;

pub type AUScheduledAudioSliceFlags = u32;

pub const kScheduledAudioSliceFlag_Complete: AUScheduledAudioSliceFlags = 0x01;
pub const kScheduledAudioSliceFlag_BeganToRender: AUScheduledAudioSliceFlags = 0x02;
pub const kScheduledAudioSliceFlag_BeganToRenderLate: AUScheduledAudioSliceFlags = 0x04;
pub const kScheduledAudioSliceFlag_Loop: AUScheduledAudioSliceFlags = 0x08;
pub const kScheduledAudioSliceFlag_Interrupt: AUScheduledAudioSliceFlags = 0x10;
pub const kScheduledAudioSliceFlag_InterruptAtLoop: AUScheduledAudioSliceFlags = 0x20;

pub type ScheduledAudioSliceCompletionProc =
    unsafe extern "C" fn(userData: *mut c_void, bufferList: ScheduledAudioSlice);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScheduledAudioSlice {
    pub mTimeStamp: AudioTimeStamp,
    pub mCompletionProc: Option<ScheduledAudioSliceCompletionProc>,
    pub mCompletionProcUserData: *mut c_void,
    pub mFlags: AUScheduledAudioSliceFlags,
    pub mReserved: u32,
    pub mReserved2: *mut c_void,
    pub mNumberFrames: u32,
    pub mBufferList: *mut AudioBufferList,
}

pub const kAudioUnitProperty_ScheduledFileIDs: AudioUnitPropertyID = 3310;
pub const kAudioUnitProperty_ScheduledFileRegion: AudioUnitPropertyID = 3311;
pub const kAudioUnitProperty_ScheduledFilePrime: AudioUnitPropertyID = 3312;
pub const kAudioUnitProperty_ScheduledFileBufferSizeFrames: AudioUnitPropertyID = 3313;
pub const kAudioUnitProperty_ScheduledFileNumberBuffers: AudioUnitPropertyID = 3314;

pub type ScheduledAudioFileRegionCompletionProc = unsafe extern "C" fn(
    userData: *mut c_void,
    fileRegion: *const ScheduledAudioFileRegion,
    result: OSStatus,
);

pub enum OpaqueAudioFileID {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScheduledAudioFileRegion {
    pub mTimeStamp: AudioTimeStamp,
    pub mCompletionProc: Option<ScheduledAudioFileRegionCompletionProc>,
    pub mCompletionProcUserData: *mut c_void,
    pub mAudioFile: *const OpaqueAudioFileID,
    pub mLoopCount: u32,
    pub mStartFrame: i64,
    pub mFramesToPlay: u32,
}

pub const kMusicDeviceProperty_UsesInternalReverb: AudioUnitPropertyID =
    kAudioUnitProperty_UsesInternalReverb;
pub const kMusicDeviceProperty_SoundBankData: AudioUnitPropertyID = 1008;
pub const kMusicDeviceProperty_StreamFromDisk: AudioUnitPropertyID = 1011;
pub const kMusicDeviceProperty_SoundBankFSRef: AudioUnitPropertyID = 1012;

pub const kMusicDeviceProperty_InstrumentName: AudioUnitPropertyID = 1001;
pub const kMusicDeviceProperty_InstrumentNumber: AudioUnitPropertyID = 1004;

pub const kMusicDeviceProperty_InstrumentCount: AudioUnitPropertyID = 1000;
pub const kMusicDeviceProperty_BankName: AudioUnitPropertyID = 1007;
pub const kMusicDeviceProperty_SoundBankURL: AudioUnitPropertyID = 1100;

pub const kAUMIDISynthProperty_EnablePreload: AudioUnitPropertyID = 4119;

pub const kAUSamplerProperty_LoadInstrument: AudioUnitPropertyID = 4102;
pub const kAUSamplerProperty_LoadAudioFiles: AudioUnitPropertyID = 4101;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AUSamplerInstrumentData {
    pub fileURL: CFURLRef,
    pub instrumentType: u8,
    pub bankMSB: u8,
    pub bankLSB: u8,
    pub presetID: u8,
}

pub const kInstrumentType_DLSPreset: u32 = 1;
pub const kInstrumentType_SF2Preset: u32 = kInstrumentType_DLSPreset;
pub const kInstrumentType_AUPreset: u32 = 2;
pub const kInstrumentType_Audiofile: u32 = 3;
pub const kInstrumentType_EXS24: u32 = 4;

pub const kAUSampler_DefaultPercussionBankMSB: u32 = 0x78;
pub const kAUSampler_DefaultMelodicBankMSB: u32 = 0x79;
pub const kAUSampler_DefaultBankLSB: u32 = 0x00;

pub const kAudioUnitProperty_DeferredRendererPullSize: AudioUnitParameterID = 3320;
pub const kAudioUnitProperty_DeferredRendererExtraLatency: AudioUnitParameterID = 3321;
pub const kAudioUnitProperty_DeferredRendererWaitFrames: AudioUnitParameterID = 3322;

pub const kAUNetReceiveProperty_Hostname: AudioUnitParameterID = 3511;
pub const kAUNetReceiveProperty_Password: AudioUnitParameterID = 3512;

pub const kAUNetSendProperty_PortNum: AudioUnitPropertyID = 3513;
pub const kAUNetSendProperty_TransmissionFormat: AudioUnitPropertyID = 3514;
pub const kAUNetSendProperty_TransmissionFormatIndex: AudioUnitPropertyID = 3515;
pub const kAUNetSendProperty_ServiceName: AudioUnitPropertyID = 3516;
pub const kAUNetSendProperty_Disconnect: AudioUnitPropertyID = 3517;
pub const kAUNetSendProperty_Password: AudioUnitPropertyID = 3518;

pub const kAUNetSendPresetFormat_PCMf32: AudioUnitPropertyID = 0;
pub const kAUNetSendPresetFormat_PCMInt24: AudioUnitPropertyID = 1;
pub const kAUNetSendPresetFormat_PCMInt16: AudioUnitPropertyID = 2;
pub const kAUNetSendPresetFormat_Lossless24: AudioUnitPropertyID = 3;
pub const kAUNetSendPresetFormat_Lossless16: AudioUnitPropertyID = 4;
pub const kAUNetSendPresetFormat_ULaw: AudioUnitPropertyID = 5;
pub const kAUNetSendPresetFormat_IMA4: AudioUnitPropertyID = 6;
pub const kAUNetSendPresetFormat_AAC_128kbpspc: AudioUnitPropertyID = 7;
pub const kAUNetSendPresetFormat_AAC_96kbpspc: AudioUnitPropertyID = 8;
pub const kAUNetSendPresetFormat_AAC_80kbpspc: AudioUnitPropertyID = 9;
pub const kAUNetSendPresetFormat_AAC_64kbpspc: AudioUnitPropertyID = 10;
pub const kAUNetSendPresetFormat_AAC_48kbpspc: AudioUnitPropertyID = 11;
pub const kAUNetSendPresetFormat_AAC_40kbpspc: AudioUnitPropertyID = 12;
pub const kAUNetSendPresetFormat_AAC_32kbpspc: AudioUnitPropertyID = 13;
pub const kAUNetSendPresetFormat_AAC_LD_64kbpspc: AudioUnitPropertyID = 14;
pub const kAUNetSendPresetFormat_AAC_LD_48kbpspc: AudioUnitPropertyID = 15;
pub const kAUNetSendPresetFormat_AAC_LD_40kbpspc: AudioUnitPropertyID = 16;
pub const kAUNetSendPresetFormat_AAC_LD_32kbpspc: AudioUnitPropertyID = 17;
pub const kAUNetSendNumPresetFormats: AudioUnitPropertyID = 18;

// AudioUnitParameters.h

pub const kAUGroupParameterID_Volume: AudioUnitParameterID = 7;
pub const kAUGroupParameterID_Sustain: AudioUnitParameterID = 64;
pub const kAUGroupParameterID_Sostenuto: AudioUnitParameterID = 66;
pub const kAUGroupParameterID_AllNotesOff: AudioUnitParameterID = 123;
pub const kAUGroupParameterID_ModWheel: AudioUnitParameterID = 1;
pub const kAUGroupParameterID_PitchBend: AudioUnitParameterID = 0xE0;
pub const kAUGroupParameterID_AllSoundOff: AudioUnitParameterID = 120;
pub const kAUGroupParameterID_ResetAllControllers: AudioUnitParameterID = 121;
pub const kAUGroupParameterID_Pan: AudioUnitParameterID = 10;
pub const kAUGroupParameterID_Foot: AudioUnitParameterID = 4;
pub const kAUGroupParameterID_ChannelPressure: AudioUnitParameterID = 0xD0;
pub const kAUGroupParameterID_KeyPressure: AudioUnitParameterID = 0xA0;
pub const kAUGroupParameterID_Expression: AudioUnitParameterID = 11;
pub const kAUGroupParameterID_DataEntry: AudioUnitParameterID = 6;
pub const kAUGroupParameterID_Volume_LSB: AudioUnitParameterID = kAUGroupParameterID_Volume + 32;
pub const kAUGroupParameterID_ModWheel_LSB: AudioUnitParameterID =
    kAUGroupParameterID_ModWheel + 32;
pub const kAUGroupParameterID_Pan_LSB: AudioUnitParameterID = kAUGroupParameterID_Pan + 32;
pub const kAUGroupParameterID_Foot_LSB: AudioUnitParameterID = kAUGroupParameterID_Foot + 32;
pub const kAUGroupParameterID_Expression_LSB: AudioUnitParameterID =
    kAUGroupParameterID_Expression + 32;
pub const kAUGroupParameterID_DataEntry_LSB: AudioUnitParameterID =
    kAUGroupParameterID_DataEntry + 32;
pub const kAUGroupParameterID_KeyPressure_FirstKey: AudioUnitParameterID = 256;
pub const kAUGroupParameterID_KeyPressure_LastKey: AudioUnitParameterID = 383;

// MusicDevice.h

pub type MusicDeviceInstrumentID = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MusicDeviceStdNoteParams {
    pub argCount: u32,
    pub mPitch: f32,
    pub mVelocity: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NoteParamsControlValue {
    pub mID: AudioUnitParameterID,
    pub mValue: AudioUnitParameterValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MusicDeviceNoteParams {
    pub argCount: u32,
    pub mPitch: f32,
    pub mVelocity: f32,
    pub mControls: [NoteParamsControlValue; 0],
}

pub const kMusicNoteEvent_UseGroupInstrument: u32 = 0xFFFFFFFF;
pub const kMusicNoteEvent_Unused: u32 = 0xFFFFFFFF;

pub type MusicDeviceGroupID = u32;
pub type NoteInstanceID = u32;

pub type MusicDeviceComponent = AudioComponentInstance;

#[cfg_attr(target_os = "macos", link(name = "AudioToolbox", kind = "framework"))]
unsafe extern "C" {
    pub fn MusicDeviceMIDIEvent(
        inUnit: MusicDeviceComponent,
        inStatus: u32,
        inData1: u32,
        inData2: u32,
        inOffsetSampleFrame: u32,
    ) -> OSStatus;

    pub fn MusicDeviceSysEx(
        inUnit: MusicDeviceComponent,
        inData: *const u8,
        inLength: u32,
    ) -> OSStatus;

    pub fn MusicDeviceStartNote(
        inUnit: MusicDeviceComponent,
        inInstrument: MusicDeviceInstrumentID,
        inGroupID: MusicDeviceGroupID,
        outNoteInstanceID: *mut NoteInstanceID,
        inOffsetSampleFrame: u32,
        inParams: *const MusicDeviceNoteParams,
    ) -> OSStatus;

    pub fn MusicDeviceStopNote(
        inUnit: MusicDeviceComponent,
        inGroupID: MusicDeviceGroupID,
        inNoteInstanceID: NoteInstanceID,
        inOffsetSampleFrame: u32,
    ) -> OSStatus;
}

pub const kMusicDeviceRange: i16 = 0x0100;
pub const kMusicDeviceMIDIEventSelect: i16 = 0x0101;
pub const kMusicDeviceSysExSelect: i16 = 0x0102;
pub const kMusicDevicePrepareInstrumentSelect: i16 = 0x0103;
pub const kMusicDeviceReleaseInstrumentSelect: i16 = 0x0104;
pub const kMusicDeviceStartNoteSelect: i16 = 0x0105;
pub const kMusicDeviceStopNoteSelect: i16 = 0x0106;

pub type MusicDeviceMIDIEventProc = unsafe extern "C" fn(
    self_: *mut c_void,
    inStatus: u32,
    inData1: u32,
    inData2: u32,
    inOffsetSampleFrame: u32,
) -> OSStatus;

pub type MusicDeviceSysExProc =
    unsafe extern "C" fn(self_: *mut c_void, inData: *const u8, inLength: u32) -> OSStatus;

pub type MusicDeviceStartNoteProc = unsafe extern "C" fn(
    self_: *mut c_void,
    inInstrument: MusicDeviceInstrumentID,
    inGroupID: MusicDeviceGroupID,
    outNoteInstanceID: *mut NoteInstanceID,
    inOffsetSampleFrame: u32,
    inParams: *const MusicDeviceNoteParams,
) -> OSStatus;

pub type MusicDeviceStopNoteProc = unsafe extern "C" fn(
    self_: *mut c_void,
    inGroupID: MusicDeviceGroupID,
    inNoteInstanceID: NoteInstanceID,
    inOffsetSampleFrame: u32,
) -> OSStatus;

// AudioUnitUtilities.h

pub const kAUParameterListener_AnyParameter: u32 = 0xFFFFFFFF;

pub const kAudioUnitEvent_ParameterValueChange: u32 = 0;
pub const kAudioUnitEvent_BeginParameterChangeGesture: u32 = 1;
pub const kAudioUnitEvent_EndParameterChangeGesture: u32 = 2;
pub const kAudioUnitEvent_PropertyChange: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitEvent {
    pub mEventType: u32,
    pub mArgument: AudioUnitEventArgument,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union AudioUnitEventArgument {
    pub mParameter: AudioUnitParameter,
    pub mProperty: AudioUnitProperty,
}

pub type AUParameterListenerProc = unsafe extern "C" fn(
    inUserData: *mut c_void,
    inObject: *mut c_void,
    inParameter: *const AudioUnitParameter,
    inValue: AudioUnitParameterValue,
);

pub type AUEventListenerProc = unsafe extern "C" fn(
    inUserData: *mut c_void,
    inObject: *mut c_void,
    inEvent: *const AudioUnitEvent,
    inEventHostTime: u64,
    inParameterValue: AudioUnitParameterValue,
);

#[repr(C)]
pub struct __AUListenerBase(c_void);

pub type AUParameterListenerRef = *const __AUListenerBase;
pub type AUEventListenerRef = AUParameterListenerRef;

#[cfg_attr(target_os = "macos", link(name = "AudioToolbox", kind = "framework"))]
unsafe extern "C" {
    pub fn AUListenerCreate(
        inProc: AUParameterListenerProc,
        inUserData: *mut c_void,
        inRunLoop: CFRunLoopRef,
        inRunLoopMode: CFStringRef,
        inNotificationInterval: f32,
        outListener: *mut AUParameterListenerRef,
    ) -> OSStatus;

    pub fn AUListenerDispose(inListener: AUParameterListenerRef) -> OSStatus;

    pub fn AUListenerAddParameter(
        inListener: AUParameterListenerRef,
        inObject: *mut c_void,
        inParameter: *const AudioUnitParameter,
    ) -> OSStatus;

    pub fn AUListenerRemoveParameter(
        inListener: AUParameterListenerRef,
        inObject: *mut c_void,
        inParameter: *const AudioUnitParameter,
    ) -> OSStatus;

    pub fn AUParameterSet(
        inSendingListener: AUParameterListenerRef,
        inSendingObject: *mut c_void,
        inParameter: *const AudioUnitParameter,
        inValue: AudioUnitParameterValue,
        inBufferOffsetInFrames: u32,
    ) -> OSStatus;

    pub fn AUParameterListenerNotify(
        inSendingListener: AUParameterListenerRef,
        inSendingObject: *mut c_void,
        inParameter: *const AudioUnitParameter,
    ) -> OSStatus;

    pub fn AUEventListenerCreate(
        inProc: AUEventListenerProc,
        inUserData: *mut c_void,
        inRunLoop: CFRunLoopRef,
        inRunLoopMode: CFStringRef,
        inNotificationInterval: f32,
        inValueChangeGranularity: f32,
        outListener: *mut AUEventListenerRef,
    ) -> OSStatus;

    pub fn AUEventListenerAddEventType(
        inListener: AUEventListenerRef,
        inObject: *mut c_void,
        inEvent: *const AudioUnitEvent,
    ) -> OSStatus;

    pub fn AUEventListenerRemoveEventType(
        inListener: AUEventListenerRef,
        inObject: *mut c_void,
        inEvent: *const AudioUnitEvent,
    ) -> OSStatus;

    pub fn AUEventListenerNotify(
        inSendingListener: AUEventListenerRef,
        inSendingObject: *mut c_void,
        inEvent: *const AudioUnitEvent,
    ) -> OSStatus;

    pub fn AUParameterValueToLinear(
        inParameterValue: AudioUnitParameterValue,
        inParameter: *const AudioUnitParameter,
    ) -> f32;

    pub fn AUParameterValueFromLinear(
        inLinearValue: f32,
        inParameter: *const AudioUnitParameter,
    ) -> AudioUnitParameterValue;

    pub fn AUParameterFormatValue(
        inParameterValue: f64,
        inParameter: *const AudioUnitParameter,
        inTextBuffer: *const c_char,
        inDigits: u32,
    );
}

use log::warn;

use crate::{AMediaCrypto, AMediaFormat, ANativeWindow, MediaFormat, MediaStatus, NativeWindow};
use std::{
    ffi::{c_void, CString},
    marker::PhantomData,
    os::raw::c_char,
    ptr::{null_mut, slice_from_raw_parts},
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BufferInfo {
    offset: i32,
    size: i32,
    presentation_time_us: i64,
    flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AMediaCodecCryptoInfo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct AMediaCodec {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[derive(Clone, Copy, Debug)]
pub enum BufferFlag {
    CodecConfig = 2,
    EndOfStream = 4,
    PartialFrame = 8,
    Encode = 1,
}

impl TryFrom<i32> for BufferFlag {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::CodecConfig),
            4 => Ok(Self::EndOfStream),
            8 => Ok(Self::PartialFrame),
            1 => Ok(Self::Encode),
            _ => Err(String::from("Not Found")),
        }
    }
}

impl TryFrom<BufferFlag> for i32 {
    type Error = String;

    fn try_from(value: BufferFlag) -> Result<Self, Self::Error> {
        Ok(value as i32)
    }
}

impl BufferFlag {
    pub fn is_contained_in(&self, flag: i32) -> bool {
        return flag & (*self as i32) > 0;
    }

    pub fn add_to_flag(&self, flag: &mut i32) {
        *flag |= *self as i32;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InfoFlag {
    OutputBuffersChanged = -3,
    OutputFormatChanged = -2,
    TryAgainLater = -1,
}

impl TryFrom<i32> for InfoFlag {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -3 => Ok(Self::OutputBuffersChanged),
            -2 => Ok(Self::OutputFormatChanged),
            -1 => Ok(Self::TryAgainLater),
            _ => Err(String::from("Not Found")),
        }
    }
}

impl TryFrom<InfoFlag> for i32 {
    type Error = String;

    fn try_from(value: InfoFlag) -> Result<Self, Self::Error> {
        Ok(value as i32)
    }
}

impl InfoFlag {
    pub fn is_contained_in(&self, flag: i32) -> bool {
        return flag & (*self as i32) > 0;
    }

    pub fn add_to_flag(&self, flag: &mut i32) {
        *flag |= *self as i32;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CryptoInfoMode {
    Clear = 0,
    AesCtr = 1,
    AesWv = 2,
    AesCbc = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CryptoInfoPattern {
    pub encrypt_blocks: i32,
    pub skip_blocks: i32,
}

type _AMediaCodecOnAsyncInputAvailable = extern "C" fn(
    // Codec
    *const AMediaCodec,
    // Userdata
    userdata: *const c_void,
    // Index
    index: i32,
);

type _AMediaCodecOnAsyncOutputAvailable = extern "C" fn(
    // Codec
    *const AMediaCodec,
    // Userdata
    *const c_void,
    // Index
    i32,
    // Buffer info
    *const BufferInfo,
);

type _AMediaCodecOnAsyncFormatChanged = extern "C" fn(
    // Codec
    *const AMediaCodec,
    // Userdata
    *const c_void,
    // Format
    *const AMediaFormat,
);

type _AMediaCodecOnAsyncError = extern "C" fn(
    // Codec
    *const AMediaCodec,
    // Userdata
    *const c_void,
    // Error
    i32,
    // Action code
    i32,
    // Details
    *const c_char,
);

#[repr(C)]
struct _AMediaCodecOnAsyncNotifyCallback {
    on_async_input_available: _AMediaCodecOnAsyncInputAvailable,
    on_async_output_available: _AMediaCodecOnAsyncOutputAvailable,
    on_async_format_changed: _AMediaCodecOnAsyncFormatChanged,
    on_async_error: _AMediaCodecOnAsyncError,
}

// FFI FUNCTIONS BEGIN

#[link(name = "mediandk")]
extern "C" {
    /// Create codec by name. Use this if you know the exact codec you want to use.
    /// When configuring, you will need to specify whether to use the codec as an encoder or decoder.
    /// <hr />
    /// Since: API 21
    fn AMediaCodec_createCodecByName(name: *const c_char) -> *mut AMediaCodec;

    /// Create codec by mime type. Most applications will use this, specifying a mime type obtained from media extractor.
    /// <hr />
    /// Since: API 21
    fn AMediaCodec_createDecoderByType(mime_type: *const c_char) -> *mut AMediaCodec;

    /// Create encoder by mime type.
    /// <hr />
    /// Since: API 21
    fn AMediaCodec_createEncoderByType(mime_type: *const c_char) -> *mut AMediaCodec;

    /// Delete the codec and free its resources
    /// <hr />
    /// Since: API 21
    fn AMediaCodec_delete(codec: *mut AMediaCodec) -> isize;

    /// Configure the codec. For decoding, you would typically get the format from an extractor
    /// <hr />
    /// Since: API 21
    fn AMediaCodec_configure(
        codec: *mut AMediaCodec,
        format: *const AMediaFormat,
        surface: *mut ANativeWindow,
        crypto: *mut AMediaCrypto,
        flags: u32,
    ) -> isize;

    /// Start the codec. A codec must be configured before it can be started, and must be started before buffers can be sent to it.
    /// <hr />
    /// Since: API 21
    fn AMediaCodec_start(codec: *mut AMediaCodec) -> isize;

    /// Stop the codec.
    /// <hr />
    /// Since: API 21
    fn AMediaCodec_stop(codec: *mut AMediaCodec) -> isize;

    /// Flush the codec's input and output. All indices previously returned from calls to `AMediaCodec_dequeueInputBuffer` and `AMediaCodec_dequeueOutpuBuffer` become invalid.
    /// <hr />
    /// Since: API 21
    fn AMediaCodec_flush(codec: *mut AMediaCodec) -> isize;

    /// Get an input buffer. The specified buffer index must have been previously obtained from dequeueInputBuffer, and not yet queued.
    /// <hr />
    /// Since: API 21
    fn AMediaCodec_getInputBuffer(
        codec: *mut AMediaCodec,
        idx: usize,
        out_size: *mut usize,
    ) -> *mut u8;

    /// Get an output buffer. The specified buffer index must have been previously obtained from `dequeueOutpuBuffer`, and not yet queued.
    /// <hr />
    /// Since: API 21
    fn AMediaCodec_getOutputBuffer(
        codec: *mut AMediaCodec,
        idx: usize,
        out_size: *mut usize,
    ) -> *mut u8;

    /// Get the index of the next available input buffer. An app will typically use this with `getInputBuffer` to get a pointer to the buffer, then copy the data to be encoded or decoded into the buffer before passing it to the codec.
    /// <hr />
    /// Since: API 21
    fn AMediaCodec_dequeueInputBuffer(codec: *mut AMediaCodec, timeout_us: i64) -> isize;

    /// Send the specified buffer to the codec for processing
    /// <hr />
    /// Since: API 21
    fn AMediaCodec_queueInputBuffer(
        codec: *mut AMediaCodec,
        idx: usize,
        offset: i32,
        size: usize,
        time: u64,
        flags: u32,
    ) -> isize;

    /// Send the specified buffer to the codec for processing
    /// <hr />
    /// Since: API 21
    fn AMediaCodec_queueSecureInputBuffer(
        codec: *mut AMediaCodec,
        idx: usize,
        offset: i32,
        info: *mut AMediaCodecCryptoInfo,
        time: u64,
        flags: u32,
    ) -> isize;

    /// Get the index of the next available buffer of processed data
    /// <hr />
    // Since: API 21
    fn AMediaCodec_dequeueOutputBuffer(
        codec: *mut AMediaCodec,
        info: *mut BufferInfo,
        timeout_us: i64,
    ) -> isize;

    /// Returns the format of the codec's output.
    /// The caller must free the returned format
    /// <hr />
    // Since: API 21
    fn AMediaCodec_getOutputFormat(codec: *mut AMediaCodec) -> *mut AMediaFormat;

    /// If you are done with a buffer, use this call to return the buffer to the codec. If you previously specified a surface when configuring this video decoder, you can optionally render the buffer.
    /// <hr />
    // Since: API 21
    fn AMediaCodec_releaseOutputBuffer(
        codec: *mut AMediaCodec,
        index: usize,
        render: bool,
    ) -> isize;

    /// Dynamically sets the output surface of a codec.
    /// This can only be used if the codec was configured with an output surface. The new output surface should have a compatible usage type to the original output surface. E.g. Codecs may not support switching from a SurfaceTexture (GPU readable) output to ImageReader (software readable) output.
    /// <hr />
    // Since: API 21
    fn AMediaCodec_setOutputSurface(codec: *mut AMediaCodec, surface: *mut ANativeWindow) -> isize;

    /// If you are done with a buffer, use this call to update its surface timestamp and return it to the codec to render it to the output surface. If you have not specified an output surface when configuring this video codec, this call will simply return the buffer to the codec.
    /// <hr />
    // Since: API 21
    fn AMediaCodec_releaseOutputBufferAtTime(
        codec: *mut AMediaCodec,
        idx: usize,
        timestamp_ns: i64,
    );

    /// Creates a surface that can be used as input to encoder, in place of input buffers.
    ///
    /// This can only be called after the codec has been configured via `AMediaCodec_configure` and before `AMediaCodec_start` has been called.
    ///
    /// The application is responsible for releasing the surface by calling `ANativeWindow_release` when done.
    ///
    /// <hr />
    /// Since: API 26
    #[cfg(feature = "api26")]
    fn AMediaCodec_createInputSurface(
        codec: *mut AMediaCodec,
        surface: *mut *mut ANativeWindow,
    ) -> isize;

    /// Creates a persistent surface that can be used as the input to encoder.
    ///
    /// Persistent surface can be reused by MediaCodec instances and can be set on a new instance via `AMediaCodec_setInputSurface`. A persistent surface can be connected to at most one instance of MediaCodec at any point in time.
    ///
    /// The application is responsible for releasing the surface by calling `ANativeWindow_release` when done.
    ///
    ///<hr />
    /// Since: API 26
    #[cfg(feature = "api26")]
    fn AMediaCodec_createPersistentInputSurface(surface: *mut *mut ANativeWindow) -> isize;

    /// Set a persistent surface that can be used as input to encoder, in place of input buffers
    ///
    /// The surface provided **must** be a persistent surface created via `AMediaCodec_createPersistentInputSurface`.
    /// This can only be called after the codec has been configured by calling `AMediaCodec_configure` and before `AMediaCodec_start` has been called.
    ///
    /// <hr />
    /// Since: API 26
    #[cfg(feature = "api26")]
    fn AMediaCodec_setInputSurface(codec: *mut AMediaCodec, surface: *mut ANativeWindow) -> isize;

    /// Signal additional parameters to the codec instance.
    ///
    /// Parameters can be communicated only when the codec is running, i.e. after `AMediaCodec_start` has been called.
    ///
    /// **NOTE:** Some of these parameter changes may silently fail to apply.
    ///
    /// <hr />
    /// Since: API 26
    #[cfg(feature = "api26")]
    fn AMediaCodec_setParameters(codec: *mut AMediaCodec, format: *const AMediaFormat) -> isize;

    /// Signals end-of-stream on input. Equivalent to submitting an empty buffer with `AMEDIACODEC_BUFFER_FLAG_END_OF_STREAM` set.
    ///
    /// Returns `AMEDIA_ERROR_INVALID_OPERATION` when used with an encoder not in executing state or not receiving input from a Surface created from `AMediaCodec_createInputSurface` or `AMediaCodec_createPersistentInputSurface`.
    ///
    /// Returns the previous codec error if one exists.
    /// Return AMEDIA_OK when completed successfully.
    ///
    /// <hr />
    /// Since: API 26
    #[cfg(feature = "api26")]
    fn AMediaCodec_signalEndOfInputStream(codec: *mut AMediaCodec) -> isize;

    /// Get format of the buffer. The specified buffer index must have been previously obtained from `dequeueOutputBuffer`.
    /// The caller must free the returned format.
    /// <br />
    /// Since: API 28
    #[cfg(feature = "api28")]
    fn AMediaCodec_getBufferFormat(codec: *mut AMediaCodec, index: usize) -> *mut AMediaFormat;

    /// Get the component name. If the codec was created by `createDecoderByType` or `createEncoderByType`, what component is chosen is not known beforehand. Caller shall call `AMediaCodec_releaseName` to free the returned pointer.
    /// <hr />
    /// Since: API 28
    #[cfg(feature = "api28")]
    fn AMediaCodec_getName(codec: *mut AMediaCodec, out_name: *mut *mut c_char) -> isize;

    /// Free the memory pointed to by name which is returned by AMediaCodec_getName.
    /// <hr />
    /// Since: API 28.
    #[cfg(feature = "api28")]
    fn AMediaCodec_releaseName(codec: *mut AMediaCodec, name: *mut c_char);

    /// Set an asynchronous callback for actionable AMediaCodec events.
    /// When asynchronous callback is enabled, the client should not call `AMediaCodec_getInputBuffer`, `AMediaCodec_getOutputBuffer`, `AMediaCodec_dequeueInputBuffer` or `AMediaCodec_dequeueOutputBuffer`.
    ///
    /// Also, `AMediaCodec_flush` behaves differently in asynchronous mode.
    /// After calling AMediaCodec_flush, you must call AMediaCodec_start to "resume" receiving input buffers, even if an input surface was created.
    ///
    /// The specified userdata is the pointer used when those callback functions are called.
    ///
    /// All callbacks are fired on one NDK internal thread.
    /// `AMediaCodec_setAsyncNotifyCallback` should not be called on the callback thread.
    /// No heavy duty task should be performed on the callback thread.
    ///
    /// <hr />
    /// Since: API 28
    #[cfg(feature = "api28")]
    fn AMediaCodec_setAsyncNotifyCallback(
        codec: *mut AMediaCodec,
        callback: _AMediaCodecOnAsyncNotifyCallback,
        userdata: *mut c_void,
    );

    /// Release the crypto if applicable.
    /// <hr />
    /// Since: API 28.
    #[cfg(feature = "api28")]
    fn AMediaCodec_releaseCrypto(codec: *mut AMediaCodec) -> isize;

    /// Call this after `AMediaCodec_configure` returns successfully to get the input format accepted by the codec. Do this to determine what optional configuration parameters were supported by the codec.
    ///
    /// The caller must free the returned format
    /// <hr />
    /// Since: API 28
    #[cfg(feature = "api28")]
    fn AMediaCodec_getInputFormat(codec: *mut AMediaCodec) -> *mut AMediaFormat;

    /// Returns true if the codec cannot proceed further, but can be recovered by stopping, configuring and starting again.
    ///
    /// <hr />
    /// Since: API 28.
    #[cfg(feature = "api28")]
    fn AMediaCodecActionCode_isRecoverable(action_code: i32) -> bool;

    /// Returns true if the codec error is a transient issue perhaps due to resource constraints, and that the method (or encoding/decoding) may be retried at a later time.
    ///
    /// <hr />
    /// Since: API 28.
    #[cfg(feature = "api28")]
    fn AMediaCodecActionCode_isTransient(action_code: i32) -> bool;

    /// Since: API 21
    fn AMediaCodecCryptoInfo_new(
        num_subsamples: i32,
        key: &[u8; 16],
        iv: &[u8; 16],
        mode: CryptoInfoMode,
        clearbytes: *mut usize,
        encrypted_bytes: *mut usize,
    ) -> *mut AMediaCodecCryptoInfo;

    /// Since: API 21
    fn AMediaCodecCryptoInfo_delete(info: *mut AMediaCodecCryptoInfo) -> isize;

    /// Since: API 21
    fn AMediaCodecCryptoInfo_setPattern(
        info: *mut AMediaCodecCryptoInfo,
        pattern: *mut CryptoInfoPattern,
    );

    /// Since: API 21
    fn AMediaCodecCryptoInfo_getNumSubSamples(info: *mut AMediaCodecCryptoInfo) -> usize;

    /// Since: API 21
    fn AMediaCodecCryptoInfo_getKey(info: *mut AMediaCodecCryptoInfo, dst: *mut u8) -> isize;

    /// Since: API 21
    fn AMediaCodecCryptoInfo_getIV(info: *mut AMediaCodecCryptoInfo, dst: *mut u8) -> isize;

    /// Since: API 21
    fn AMediaCodecCryptoInfo_getMode(info: *mut AMediaCodecCryptoInfo) -> CryptoInfoMode;

    /// Since: API 21
    fn AMediaCodecCryptoInfo_getClearBytes(
        info: *mut AMediaCodecCryptoInfo,
        dst: *mut usize,
    ) -> isize;

    /// Since: API 21
    fn AMediaCodecCryptoInfo_getEncryptedBytes(
        info: *mut AMediaCodecCryptoInfo,
        dst: *mut usize,
    ) -> isize;
}
// FFI FUNCTIONS END

pub struct CodecInputBuffer<'a> {
    pub(crate) _marker: PhantomData<&'a (*mut u8, core::marker::PhantomPinned)>,
    pub(crate) buffer: *mut u8,
    pub(crate) size: usize,
    pub(crate) write_size: usize,
    index: usize,
    codec: *mut AMediaCodec,
    pub(crate) time: u64,
    pub(crate) flags: u32,
}

impl CodecInputBuffer<'_> {
    fn new(codec: *mut AMediaCodec, index: usize, buffer: *mut u8, size: usize) -> Self {
        Self {
            _marker: PhantomData,
            buffer,
            size,
            index,
            codec,
            write_size: 0,
            time: 0,
            flags: 0,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn write_size(&self) -> usize {
        self.write_size
    }

    pub fn buffer(&self) -> (*mut u8, usize) {
        (self.buffer, self.size)
    }

    pub fn set_time(&mut self, time: u64) {
        self.time = time;
    }

    pub fn set_flags(&mut self, flags: u32) {
        self.flags = flags;
    }

    pub fn set_write_size(&mut self, write_size: usize) {
        self.write_size = write_size;
    }
}

impl Drop for CodecInputBuffer<'_> {
    fn drop(&mut self) {
        unsafe {
            AMediaCodec_queueInputBuffer(
                self.codec,
                self.index,
                0,
                self.write_size,
                self.time,
                self.flags,
            );
        }
    }
}

pub struct CodecOutputBuffer<'a> {
    _marker: PhantomData<&'a (*mut u8, core::marker::PhantomPinned)>,
    codec: *mut AMediaCodec,
    info: BufferInfo,
    index: usize,
    using_buffers: bool,
    buffer: *mut u8,
    _size: usize,
    format: MediaFormat,
    render: bool,
}

impl CodecOutputBuffer<'_> {
    fn new(
        codec: *mut AMediaCodec,
        info: BufferInfo,
        index: usize,
        using_buffers: bool,
        buffer: *mut u8,
        size: usize,
        format: MediaFormat,
    ) -> Self {
        Self {
            codec,
            info,
            index,
            using_buffers,
            buffer,
            _size: size,
            _marker: PhantomData,
            render: false,
            format,
        }
    }

    pub fn info(&self) -> &BufferInfo {
        &self.info
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn using_buffers(&self) -> bool {
        self.using_buffers
    }

    pub fn format(&self) -> &MediaFormat {
        &self.format
    }

    pub fn buffer_slice(&self) -> Option<&[u8]> {
        if !self.using_buffers {
            return None;
        }

        unsafe {
            // Return the size of the readable buffer, instead of the buffer size itself.
            // Returning the entire buffer size is useless for the output buffer, as we only need to read data from it
            Some(&*slice_from_raw_parts(
                (self.buffer as i32 + self.info.offset) as *mut u8,
                self.info.size as usize,
            ))
        }
    }

    /// Returns audio samples if the buffer type is audio and None otherwise
    pub fn audio_samples(&self) {}

    /// Set whether this buffer should render when it gets dropped.
    /// This only works for video decoder buffers with a surface attached
    pub fn set_render(&mut self, render: bool) {
        self.render = render;
    }
}

impl Drop for CodecOutputBuffer<'_> {
    fn drop(&mut self) {
        unsafe {
            AMediaCodec_releaseOutputBuffer(self.codec, self.index, self.render);
        }
    }
}

#[derive(Debug)]
pub struct MediaCodec<'a> {
    inner: *mut AMediaCodec,
    _marker: PhantomData<&'a *const u8>,
    using_buffers: bool,
}

impl<'a> MediaCodec<'a> {
    fn from_ptr(ptr: *mut AMediaCodec) -> Self {
        Self {
            inner: ptr,
            _marker: PhantomData,
            using_buffers: false,
        }
    }

    pub fn new(name: &str) -> Option<Self> {
        unsafe {
            let name = CString::new(name).unwrap();
            let codec = AMediaCodec_createCodecByName(name.as_ptr());

            if codec.is_null() {
                return None;
            }

            Some(Self::from_ptr(codec))
        }
    }

    pub fn create_decoder(mime_type: &str) -> Option<Self> {
        unsafe {
            let mime_type = CString::new(mime_type).unwrap();
            let codec = AMediaCodec_createDecoderByType(mime_type.as_ptr());

            if codec.is_null() {
                warn!("Codec is null: {codec:p}");
                return None;
            }

            Some(Self::from_ptr(codec))
        }
    }

    pub fn create_encoder(mime_type: &str) -> Option<Self> {
        unsafe {
            let mime_type = CString::new(mime_type).unwrap();
            let codec = AMediaCodec_createEncoderByType(mime_type.as_ptr());

            if codec.is_null() {
                return None;
            }

            Some(Self::from_ptr(codec))
        }
    }

    pub fn init(
        &mut self,
        format: &MediaFormat,
        surface: Option<NativeWindow>,
        flags: u32,
    ) -> Result<(), MediaStatus> {
        unsafe {
            // configure

            let surface = if surface.is_some() {
                self.using_buffers = false;
                surface.unwrap().inner
            } else {
                self.using_buffers = true;
                null_mut()
            };

            MediaStatus::make_result(AMediaCodec_configure(
                self.inner,
                format.inner,
                surface,
                null_mut(),
                flags,
            ))
            .map(|_value| ())
        }
    }

    pub fn start(&mut self) -> Result<(), MediaStatus> {
        unsafe { MediaStatus::make_result(AMediaCodec_start(self.inner)).map(|_value| ()) }
    }

    /// **WARNING**
    ///
    /// Make sure you have released all pending buffers before calling this function
    pub fn stop(&mut self) -> Result<(), MediaStatus> {
        unsafe { MediaStatus::make_result(AMediaCodec_stop(self.inner)).map(|_| ()) }
    }

    /// **WARNING**
    ///
    /// Make sure you have released all pending buffers before calling this function
    pub fn flush(&mut self) -> Result<(), MediaStatus> {
        unsafe { MediaStatus::make_result(AMediaCodec_flush(self.inner)).map(|_| ()) }
    }

    pub fn output_format(&self) -> Option<MediaFormat> {
        unsafe {
            let format = AMediaCodec_getOutputFormat(self.inner);
            if format.is_null() {
                return None;
            }

            Some(MediaFormat::from_raw(format))
        }
    }

    pub fn set_output_surface(&mut self, window: NativeWindow) -> bool {
        if self.using_buffers {
            return false;
        }

        unsafe {
            AMediaCodec_setOutputSurface(self.inner, window.inner);
            true
        }
    }

    pub fn dequeue_input(&mut self) -> Result<CodecInputBuffer, MediaStatus> {
        unsafe {
            // 100us wait time is not too much, right?
            let index = AMediaCodec_dequeueInputBuffer(self.inner, 100);

            if index >= 0 {
                let mut out_size = 0;
                let buffer = AMediaCodec_getInputBuffer(self.inner, index as usize, &mut out_size);

                if buffer.is_null() {
                    // Return the buffer to the codec, it's not valid
                    AMediaCodec_queueInputBuffer(self.inner, index as usize, 0, 0, 0, 0);
                    warn!("Got an index with a null input buffer! What is going on here??? Index: {index}");
                    return Err(MediaStatus::ErrorUnknown);
                }

                let buf = CodecInputBuffer::new(self.inner, index as usize, buffer, out_size);

                return Ok(buf);
            }

            Err(MediaStatus::try_from(index).unwrap_or(MediaStatus::ErrorUnknown))
        }
    }

    pub fn dequeue_output(&mut self) -> Result<CodecOutputBuffer, MediaStatus> {
        unsafe {
            let mut info = BufferInfo::default();
            let index = AMediaCodec_dequeueOutputBuffer(self.inner, &mut info, 100);
            let mut out_size = 0;

            if index >= 0 {
                let mut buffer = null_mut();
                if self.using_buffers {
                    buffer = AMediaCodec_getOutputBuffer(self.inner, index as usize, &mut out_size);

                    if buffer.is_null() {
                        AMediaCodec_releaseOutputBuffer(self.inner, index as usize, false);
                        return Err(MediaStatus::ErrorUnknown);
                    }
                }

                let codec_buffer = CodecOutputBuffer::new(
                    self.inner,
                    info,
                    index as usize,
                    self.using_buffers,
                    buffer,
                    out_size,
                    self.output_format().unwrap(),
                );

                return Ok(codec_buffer);
            }

            Err(MediaStatus::ErrorUnknown)
        }
    }
}

impl<'a> Drop for MediaCodec<'a> {
    fn drop(&mut self) {
        unsafe {
            AMediaCodec_delete(self.inner);
        }
    }
}

unsafe impl<'a> Send for MediaCodec<'a> {}
unsafe impl<'a> Sync for MediaCodec<'a> {}
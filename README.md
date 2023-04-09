# voice sample analysis

- preprocessing
  - perform voice activity detection (VAD) to separate speech from silence or background noise.
  - segment the voice signal into overlapping frames (e.g., 20-30 ms with 50% overlap), and apply a window function (e.g., Hamming or Hanning) to each frame
- feature extraction
  - extract spectral features like Mel-Frequency Cepstral Coefficients (MFCCs) or Linear Predictive Coding (LPC) coefficients from each frame. these features capture the spectral envelope of the speech signal, which is essential for voice conversion.
  - extract pitch (F0) from each frame to capture the prosodic information. you can use algorithms like YIN, RAPT, or CREPE for pitch estimation.
  - optionally, extract other features like formant frequencies or aperiodicity measures that can help improve the conversion quality.
- alignment
  - align the source and target voice samples in terms of phonetic or linguistic content using dynamic time warping (DTW) or hidden Markov models (HMMs). this alignment is necessary to ensure that corresponding frames in both source and target voices are compared and processed during conversion.
- feature transformation
  - model the relationship between source and target voice features using Gaussian Mixture Models (GMMs), deep neural networks (DNNs), or other appropriate methods. train the model on the aligned features to learn the mapping between the source and target spectral features.
  - analyze the pitch difference between the source and target voices. you might find it useful to compute the mean and standard deviation of the pitch values in each voice and use these statistics to transform the source pitch to match the target pitch.

## each preprocessing step has a specific purpose

- framing the signal  
  voice signals exhibit short-term stationarity, which means that their spectral characteristics remain relatively constant over short periods (around 20-30 ms). by segmenting the signal into overlapping frames, we can analyze and process each frame independently, assuming the spectral characteristics within a frame are stationary. this assumption simplifies the feature extraction and conversion processes.

- applying a window function  
  window functions, such as Hamming or Hanning, are used to taper the edges of each frame to minimize discontinuities that may occur due to framing. these discontinuities can cause artifacts in the frequency domain representation of the signal, which would negatively impact the spectral analysis and feature extraction.

  - the Hamming window has a wider main lobe compared to the Hanning window, which results in a slightly lower frequency resolution. however, it has smaller side lobes, which means less spectral leakage. the Hamming window is often used in speech processing applications as it provides a good balance between frequency resolution and spectral leakage.

  - the Hanning window has a narrower main lobe than the Hamming window, resulting in better frequency resolution. however, its side lobes are larger, causing more spectral leakage. the Hanning window is also commonly used in speech processing, and it may provide better results in some applications.

_for voice conversion applications, the Hamming window is generally a suitable choice due to its balance between frequency resolution and spectral leakage_

the preprocessing steps of framing the signal and applying a window function prepare the voice signal for further analysis and processing in the voice conversion pipeline. these steps make it easier to extract meaningful spectral and prosodic features from the signal and apply appropriate transformations during the conversion process.

## feature extraction

- magnitude spectrum
  the magnitude spectrum is computed by applying the Short-Time Fourier Transform (STFT) to the preprocessed signal. it provides the frequency-domain representation of the signal, which allows us to analyze the spectral content of the voice. in voice conversion, it is essential to capture the spectral characteristics of the source and target speakers, as they play a crucial role in determining the perceived speaker identity.
- Mel-Frequency Cepstral Coefficients (MFCC)
  MFCCs are a compact representation of the spectral envelope, capturing the most important features of the voice signal. the computation of MFCCs involves multiple steps:
  - Mel Filter Bank: the Mel filter bank is used to convert the linear magnitude spectrum into a Mel-scaled spectrum. this scaling approximates the human auditory system's frequency resolution, emphasizing the frequency bands that are more perceptually relevant. in voice conversion, this helps to focus on the frequency regions that are more important for speaker identity.
  - logarith: applying the logarithm to the Mel-scaled spectrum helps to emphasize the spectral peaks and de-emphasize the valleys. this step further approximates the human auditory system's response, which is more sensitive to relative changes in amplitude than to absolute values.
  - Discrete Cosine Transform (DCT): the DCT is applied to the log Mel-scaled spectrum to decorrelate the spectral features, which results in a compact and robust representation of the spectral envelope. the lower-order MFCCs capture the overall spectral shape, while the higher-order MFCCs capture the detailed spectral features. in voice conversion, the lower-order MFCCs are typically more important for preserving the speaker's identity.

// Generated by gir (https://github.com/gtk-rs/gir @ ba0ee13f6f1e)
// from gir-files (https://github.com/gtk-rs/gir-files @ b827978e7d18)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ f8c393377c4e)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) ADDITIONAL_INFO_PAGE);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_ADVANCED_CODEC_DIGITAL_RADIO_SOUND);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_ADVANCED_CODEC_HD_DIGITAL_TELEVISION);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_ADVANCED_CODEC_HD_NVOD_REFERENCE);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_ADVANCED_CODEC_HD_NVOD_TIME_SHIFTED);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_ADVANCED_CODEC_MOSAIC);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_ADVANCED_CODEC_SD_DIGITAL_TELEVISION);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_ADVANCED_CODEC_SD_NVOD_REFERENCE);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_ADVANCED_CODEC_SD_NVOD_TIME_SHIFTED);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_ADVANCED_CODEC_STEREO_HD_DIGITAL_TELEVISION);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_ADVANCED_CODEC_STEREO_HD_NVOD_REFERENCE);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_ADVANCED_CODEC_STEREO_HD_NVOD_TIME_SHIFTED);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_DATA_BROADCAST);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_DIGITAL_RADIO_SOUND);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_DIGITAL_TELEVISION);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_DVB_MHP);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_DVB_SRM);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_FM_RADIO);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_MOSAIC);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_MPEG2_HD_DIGITAL_TELEVISION);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_NVOD_REFERENCE);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_NVOD_TIME_SHIFTED);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_RCS_FLS);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_RCS_MAP);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_RESERVED_00);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_RESERVED_09);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_RESERVED_0D_COMMON_INTERFACE);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_RESERVED_FF);
    PRINT_CONSTANT((gint) GST_DVB_SERVICE_TELETEXT);
    PRINT_CONSTANT((gint) GST_MPEGTS_ATSC_MGT_TABLE_TYPE_EIT0);
    PRINT_CONSTANT((gint) GST_MPEGTS_ATSC_MGT_TABLE_TYPE_EIT127);
    PRINT_CONSTANT((gint) GST_MPEGTS_ATSC_MGT_TABLE_TYPE_ETT0);
    PRINT_CONSTANT((gint) GST_MPEGTS_ATSC_MGT_TABLE_TYPE_ETT127);
    PRINT_CONSTANT((gint) GST_MPEGTS_AUDIO_TYPE_CLEAN_EFFECTS);
    PRINT_CONSTANT((gint) GST_MPEGTS_AUDIO_TYPE_HEARING_IMPAIRED);
    PRINT_CONSTANT((gint) GST_MPEGTS_AUDIO_TYPE_UNDEFINED);
    PRINT_CONSTANT((gint) GST_MPEGTS_AUDIO_TYPE_VISUAL_IMPAIRED_COMMENTARY);
    PRINT_CONSTANT((gint) GST_MPEGTS_CABLE_OUTER_FEC_NONE);
    PRINT_CONSTANT((gint) GST_MPEGTS_CABLE_OUTER_FEC_RS_204_188);
    PRINT_CONSTANT((gint) GST_MPEGTS_CABLE_OUTER_FEC_UNDEFINED);
    PRINT_CONSTANT((gint) GST_MPEGTS_CONTENT_ARTS_CULTURE);
    PRINT_CONSTANT((gint) GST_MPEGTS_CONTENT_CHILDREN_YOUTH_PROGRAM);
    PRINT_CONSTANT((gint) GST_MPEGTS_CONTENT_EDUCATION_SCIENCE_FACTUAL);
    PRINT_CONSTANT((gint) GST_MPEGTS_CONTENT_LEISURE_HOBBIES);
    PRINT_CONSTANT((gint) GST_MPEGTS_CONTENT_MOVIE_DRAMA);
    PRINT_CONSTANT((gint) GST_MPEGTS_CONTENT_MUSIC_BALLET_DANCE);
    PRINT_CONSTANT((gint) GST_MPEGTS_CONTENT_NEWS_CURRENT_AFFAIRS);
    PRINT_CONSTANT((gint) GST_MPEGTS_CONTENT_SHOW_GAME_SHOW);
    PRINT_CONSTANT((gint) GST_MPEGTS_CONTENT_SOCIAL_POLITICAL_ECONOMICS);
    PRINT_CONSTANT((gint) GST_MPEGTS_CONTENT_SPECIAL_CHARACTERISTICS);
    PRINT_CONSTANT((gint) GST_MPEGTS_CONTENT_SPORTS);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_CA_REPLACEMENT);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_DATA_BROADCAST);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_EPG);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_EVENT);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_EXTENDED_EVENT);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_HAND_OVER_ASSOCIATED);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_HAND_OVER_IDENTICAL);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_HAND_OVER_LOCAL_VARIATION);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_HAND_OVER_RESERVED);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_INFORMATION);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_IP_MAC_NOTIFICATION);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_MOBILE_HAND_OVER);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_RCS_MAP);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_RESERVED_00);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_SERVICE_REPLACEMENT);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_SYSTEM_SOFTWARE_UPDATE);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_TS_CONTAINING_COMPLETE_SI);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_TS_CONTAINING_INT);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_LINKAGE_TS_CONTAINING_SSU);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_SCRAMBLING_MODE_ATIS_0);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_SCRAMBLING_MODE_ATIS_F);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_SCRAMBLING_MODE_CISSA);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_SCRAMBLING_MODE_CSA1);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_SCRAMBLING_MODE_CSA2);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_SCRAMBLING_MODE_CSA3_FULL_ENHANCED);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_SCRAMBLING_MODE_CSA3_MINIMAL_ENHANCED);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_SCRAMBLING_MODE_CSA3_STANDARD);
    PRINT_CONSTANT((gint) GST_MPEGTS_DVB_SCRAMBLING_MODE_RESERVED);
    PRINT_CONSTANT((gint) GST_MPEGTS_FEC_1_2);
    PRINT_CONSTANT((gint) GST_MPEGTS_FEC_2_3);
    PRINT_CONSTANT((gint) GST_MPEGTS_FEC_2_5);
    PRINT_CONSTANT((gint) GST_MPEGTS_FEC_3_4);
    PRINT_CONSTANT((gint) GST_MPEGTS_FEC_3_5);
    PRINT_CONSTANT((gint) GST_MPEGTS_FEC_4_5);
    PRINT_CONSTANT((gint) GST_MPEGTS_FEC_5_6);
    PRINT_CONSTANT((gint) GST_MPEGTS_FEC_6_7);
    PRINT_CONSTANT((gint) GST_MPEGTS_FEC_7_8);
    PRINT_CONSTANT((gint) GST_MPEGTS_FEC_8_9);
    PRINT_CONSTANT((gint) GST_MPEGTS_FEC_9_10);
    PRINT_CONSTANT((gint) GST_MPEGTS_FEC_AUTO);
    PRINT_CONSTANT((gint) GST_MPEGTS_FEC_NONE);
    PRINT_CONSTANT((gint) GST_MPEGTS_GUARD_INTERVAL_19_128);
    PRINT_CONSTANT((gint) GST_MPEGTS_GUARD_INTERVAL_19_256);
    PRINT_CONSTANT((gint) GST_MPEGTS_GUARD_INTERVAL_1_128);
    PRINT_CONSTANT((gint) GST_MPEGTS_GUARD_INTERVAL_1_16);
    PRINT_CONSTANT((gint) GST_MPEGTS_GUARD_INTERVAL_1_32);
    PRINT_CONSTANT((gint) GST_MPEGTS_GUARD_INTERVAL_1_4);
    PRINT_CONSTANT((gint) GST_MPEGTS_GUARD_INTERVAL_1_8);
    PRINT_CONSTANT((gint) GST_MPEGTS_GUARD_INTERVAL_AUTO);
    PRINT_CONSTANT((gint) GST_MPEGTS_GUARD_INTERVAL_PN420);
    PRINT_CONSTANT((gint) GST_MPEGTS_GUARD_INTERVAL_PN595);
    PRINT_CONSTANT((gint) GST_MPEGTS_GUARD_INTERVAL_PN945);
    PRINT_CONSTANT((gint) GST_MPEGTS_HIERARCHY_1);
    PRINT_CONSTANT((gint) GST_MPEGTS_HIERARCHY_2);
    PRINT_CONSTANT((gint) GST_MPEGTS_HIERARCHY_4);
    PRINT_CONSTANT((gint) GST_MPEGTS_HIERARCHY_AUTO);
    PRINT_CONSTANT((gint) GST_MPEGTS_HIERARCHY_NONE);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_APSK_16);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_APSK_32);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_DQPSK);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_NONE);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_PSK_8);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_QAM_128);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_QAM_16);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_QAM_256);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_QAM_32);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_QAM_4_NR_);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_QAM_64);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_QAM_AUTO);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_QPSK);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_VSB_16);
    PRINT_CONSTANT((gint) GST_MPEGTS_MODULATION_VSB_8);
    PRINT_CONSTANT((gint) GST_MPEGTS_POLARIZATION_CIRCULAR_LEFT);
    PRINT_CONSTANT((gint) GST_MPEGTS_POLARIZATION_CIRCULAR_RIGHT);
    PRINT_CONSTANT((gint) GST_MPEGTS_POLARIZATION_LINEAR_HORIZONTAL);
    PRINT_CONSTANT((gint) GST_MPEGTS_POLARIZATION_LINEAR_VERTICAL);
    PRINT_CONSTANT((gint) GST_MPEGTS_ROLLOFF_20);
    PRINT_CONSTANT((gint) GST_MPEGTS_ROLLOFF_25);
    PRINT_CONSTANT((gint) GST_MPEGTS_ROLLOFF_35);
    PRINT_CONSTANT((gint) GST_MPEGTS_ROLLOFF_AUTO);
    PRINT_CONSTANT((gint) GST_MPEGTS_ROLLOFF_RESERVED);
    PRINT_CONSTANT((gint) GST_MPEGTS_RUNNING_STATUS_NOT_RUNNING);
    PRINT_CONSTANT((gint) GST_MPEGTS_RUNNING_STATUS_OFF_AIR);
    PRINT_CONSTANT((gint) GST_MPEGTS_RUNNING_STATUS_PAUSING);
    PRINT_CONSTANT((gint) GST_MPEGTS_RUNNING_STATUS_RUNNING);
    PRINT_CONSTANT((gint) GST_MPEGTS_RUNNING_STATUS_STARTS_IN_FEW_SECONDS);
    PRINT_CONSTANT((gint) GST_MPEGTS_RUNNING_STATUS_UNDEFINED);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_ATSC_CVCT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_ATSC_EIT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_ATSC_ETT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_ATSC_MGT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_ATSC_RRT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_ATSC_STT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_ATSC_TVCT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_BAT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_CAT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_EIT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_NIT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_PAT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_PMT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_SCTE_SIT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_SDT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_SIT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_TDT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_TOT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_TSDT);
    PRINT_CONSTANT((gint) GST_MPEGTS_SECTION_UNKNOWN);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_CONTENT_AAC);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_CONTENT_AC_3);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_CONTENT_AVC);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_CONTENT_DTS);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_CONTENT_MPEG1_LAYER2_AUDIO);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_CONTENT_MPEG2_VIDEO);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_CONTENT_SRM_CPCM);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_CONTENT_TELETEXT_OR_SUBTITLE);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_ATSC_AUDIO_AC3);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_ATSC_AUDIO_DTS_HD);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_ATSC_AUDIO_EAC3);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_ATSC_DCII_VIDEO);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_ATSC_ISOCH_DATA);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_ATSC_SIT);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_ATSC_SUBTITLING);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_AUDIO_AAC_ADTS);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_AUDIO_AAC_CLEAN);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_AUDIO_AAC_LATM);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_AUDIO_MPEG1);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_AUDIO_MPEG2);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_AUXILIARY);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_DSMCC_A);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_DSMCC_B);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_DSMCC_C);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_DSMCC_D);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_DSM_CC);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_HDMV_AUDIO_AC3);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_HDMV_AUDIO_AC3_PLUS);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_HDMV_AUDIO_AC3_PLUS_SECONDARY);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_HDMV_AUDIO_AC3_TRUE_HD);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_HDMV_AUDIO_DTS);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_HDMV_AUDIO_DTS_HD);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_HDMV_AUDIO_DTS_HD_MASTER_AUDIO);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_HDMV_AUDIO_DTS_HD_SECONDARY);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_HDMV_AUDIO_EAC3);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_HDMV_AUDIO_LPCM);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_HDMV_IGS);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_HDMV_SUBPICTURE_PGS);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_HDMV_SUBTITLE);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_H_222_1);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_IPMP_STREAM);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_METADATA_DATA_CAROUSEL);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_METADATA_OBJECT_CAROUSEL);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_METADATA_PES_PACKETS);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_METADATA_SECTIONS);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_METADATA_SYNCHRONIZED_DOWNLOAD);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_MHEG);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_MPEG2_IPMP);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_MPEG4_TIMED_TEXT);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_PRIVATE_PES_PACKETS);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_PRIVATE_SECTIONS);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_RESERVED_00);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_SCTE_ASYNC_DATA);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_SCTE_DSMCC_DCB);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_SCTE_DST_NRT);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_SCTE_ISOCH_DATA);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_SCTE_SIGNALING);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_SCTE_SIT);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_SCTE_SUBTITLING);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_SCTE_SYNC_DATA);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_SL_FLEXMUX_PES_PACKETS);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_SL_FLEXMUX_SECTIONS);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_SYNCHRONIZED_DOWNLOAD);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_USER_PRIVATE_EA);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_VIDEO_H264);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_VIDEO_H264_MVC_SUB_BITSTREAM);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_VIDEO_H264_STEREO_ADDITIONAL_VIEW);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_VIDEO_H264_SVC_SUB_BITSTREAM);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_VIDEO_HEVC);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_VIDEO_JP2K);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_VIDEO_MPEG1);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_VIDEO_MPEG2);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_VIDEO_MPEG2_STEREO_ADDITIONAL_VIEW);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_VIDEO_MPEG4);
    PRINT_CONSTANT((gint) GST_MPEGTS_STREAM_TYPE_VIDEO_RVC);
    PRINT_CONSTANT((gint) GST_MPEGTS_TRANSMISSION_MODE_16K);
    PRINT_CONSTANT((gint) GST_MPEGTS_TRANSMISSION_MODE_1K);
    PRINT_CONSTANT((gint) GST_MPEGTS_TRANSMISSION_MODE_2K);
    PRINT_CONSTANT((gint) GST_MPEGTS_TRANSMISSION_MODE_32K);
    PRINT_CONSTANT((gint) GST_MPEGTS_TRANSMISSION_MODE_4K);
    PRINT_CONSTANT((gint) GST_MPEGTS_TRANSMISSION_MODE_8K);
    PRINT_CONSTANT((gint) GST_MPEGTS_TRANSMISSION_MODE_AUTO);
    PRINT_CONSTANT((gint) GST_MPEGTS_TRANSMISSION_MODE_C1);
    PRINT_CONSTANT((gint) GST_MPEGTS_TRANSMISSION_MODE_C3780);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_AC3);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_CAPTION_SERVICE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_COMPONENT_NAME);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_CONTENT_ADVISORY);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_CRC32);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_DATA_SERVICE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_DCC_ARRIVING_REQUEST);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_DCC_DEPARTING_REQUEST);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_DOWNLOAD_DESCRIPTOR);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_EAC3);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_ENHANCED_SIGNALING);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_EXTENDED_CHANNEL_NAME);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_GENRE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_GROUP_LINK);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_MODULE_LINK);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_MULTIPROTOCOL_ENCAPSULATION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_PID_COUNT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_PRIVATE_INFORMATION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_REDISTRIBUTION_CONTROL);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_SERVICE_LOCATION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_STUFFING);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ATSC_TIME_SHIFTED_SERVICE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_AUDIO_STREAM);
    PRINT_CONSTANT((gint) GST_MTS_DESC_AUXILIARY_VIDEO_STREAM);
    PRINT_CONSTANT((gint) GST_MTS_DESC_AVC_TIMING_AND_HRD);
    PRINT_CONSTANT((gint) GST_MTS_DESC_AVC_VIDEO);
    PRINT_CONSTANT((gint) GST_MTS_DESC_CA);
    PRINT_CONSTANT((gint) GST_MTS_DESC_CONTENT_LABELING);
    PRINT_CONSTANT((gint) GST_MTS_DESC_COPYRIGHT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DATA_STREAM_ALIGNMENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DSMCC_ASSOCIATION_TAG);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DSMCC_CAROUSEL_IDENTIFIER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DSMCC_DEFERRED_ASSOCIATION_TAG);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DSMCC_NPT_ENDPOINT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DSMCC_NPT_REFERENCE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DSMCC_STREAM_EVENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DSMCC_STREAM_MODE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DTG_LOGICAL_CHANNEL);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_AAC);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_AC3);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_ADAPTATION_FIELD_DATA);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_ANCILLARY_DATA);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_ANNOUNCEMENT_SUPPORT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_APPLICATION_SIGNALLING);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_BOUQUET_NAME);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_CABLE_DELIVERY_SYSTEM);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_CA_IDENTIFIER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_CELL_FREQUENCY_LINK);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_CELL_LIST);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_COMPONENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_CONTENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_CONTENT_IDENTIFIER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_COUNTRY_AVAILABILITY);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_DATA_BROADCAST);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_DATA_BROADCAST_ID);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_DEFAULT_AUTHORITY);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_DSNG);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_DTS);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_ECM_REPETITION_RATE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_ENHANCED_AC3);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_EXTENDED_EVENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_EXTENSION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_FREQUENCY_LIST);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_FTA_CONTENT_MANAGEMENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_LINKAGE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_LOCAL_TIME_OFFSET);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_MOSAIC);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_MULTILINGUAL_BOUQUET_NAME);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_MULTILINGUAL_COMPONENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_MULTILINGUAL_NETWORK_NAME);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_MULTILINGUAL_SERVICE_NAME);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_NETWORK_NAME);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_NVOD_REFERENCE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_PARENTAL_RATING);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_PARTIAL_TRANSPORT_STREAM);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_PDC);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_PRIVATE_DATA_SPECIFIER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_RELATED_CONTENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_S2_SATELLITE_DELIVERY_SYSTEM);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_SATELLITE_DELIVERY_SYSTEM);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_SCRAMBLING);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_SERVICE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_SERVICE_AVAILABILITY);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_SERVICE_IDENTIFIER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_SERVICE_LIST);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_SERVICE_MOVE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_SHORT_EVENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_SHORT_SMOOTHING_BUFFER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_STREAM_IDENTIFIER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_STUFFING);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_SUBTITLING);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_TELEPHONE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_TELETEXT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_TERRESTRIAL_DELIVERY_SYSTEM);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_TIMESLICE_FEC_IDENTIFIER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_TIME_SHIFTED_EVENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_TIME_SHIFTED_SERVICE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_TRANSPORT_STREAM);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_TVA_ID);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_VBI_DATA);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_VBI_TELETEXT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_DVB_XAIT_LOCATION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXTERNAL_ES_ID);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_AC4);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_AUDIO_PRESELECTION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_C2_DELIVERY_SYSTEM);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_CP);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_CPCM_DELIVERY_SIGNALLING);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_CP_IDENTIFIER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_DTS_HD_AUDIO_STREAM);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_DTS_NEUTRAL);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_IMAGE_ICON);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_MESSAGE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_NETWORK_CHANGE_NOTIFY);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_SERVICE_RELOCATED);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_SH_DELIVERY_SYSTEM);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_SUPPLEMENTARY_AUDIO);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_T2MI);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_T2_DELIVERY_SYSTEM);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_TARGET_REGION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_TARGET_REGION_NAME);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_URI_LINKAGE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_VIDEO_DEPTH_RANGE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_EXT_DVB_XAIT_PID);
    PRINT_CONSTANT((gint) GST_MTS_DESC_FLEX_MUX_TIMING);
    PRINT_CONSTANT((gint) GST_MTS_DESC_FMC);
    PRINT_CONSTANT((gint) GST_MTS_DESC_FMX_BUFFER_SIZE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_HIERARCHY);
    PRINT_CONSTANT((gint) GST_MTS_DESC_IBP);
    PRINT_CONSTANT((gint) GST_MTS_DESC_IOD);
    PRINT_CONSTANT((gint) GST_MTS_DESC_IPMP);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_AUDIO_COMPONENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_BASIC_LOCAL_EVENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_BOARD_INFORMATION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_BROADCASTER_NAME);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_CA_CONTRACT_INFORMATION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_CA_EMM_TS);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_CA_SERVICE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_COMPONENT_GROUP);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_CONNECTED_TRANSMISSION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_CONTENT_AVAILABILITY);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_DATA_CONTENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_DIGITAL_COPY_CONTROL);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_DOWNLOAD_CONTENT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_EVENT_GROUP);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_EXTENDED_BROADCASTER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_HIERARCHICAL_TRANSMISSION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_HYPERLINK);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_LDT_LINKAGE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_LOGO_TRANSMISSION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_NETWORK_IDENTIFICATION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_NODE_RELATION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_PARTIAL_TS_TIME);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_REFERENCE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_SERIES);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_SERVICE_GROUP);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_SHORT_NODE_INFORMATION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_SI_PARAMETER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_SI_PRIME_TS);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_STC_REFERENCE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_TARGET_REGION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_TS_INFORMATION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISDB_VIDEO_DECODE_CONTROL);
    PRINT_CONSTANT((gint) GST_MTS_DESC_ISO_639_LANGUAGE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_J2K_VIDEO);
    PRINT_CONSTANT((gint) GST_MTS_DESC_MAXIMUM_BITRATE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_METADATA);
    PRINT_CONSTANT((gint) GST_MTS_DESC_METADATA_POINTER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_METADATA_STD);
    PRINT_CONSTANT((gint) GST_MTS_DESC_MPEG2_AAC_AUDIO);
    PRINT_CONSTANT((gint) GST_MTS_DESC_MPEG2_STEREOSCOPIC_VIDEO_FORMAT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_MPEG4_AUDIO);
    PRINT_CONSTANT((gint) GST_MTS_DESC_MPEG4_AUDIO_EXTENSION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_MPEG4_TEXT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_MPEG4_VIDEO);
    PRINT_CONSTANT((gint) GST_MTS_DESC_MULTIPLEX_BUFFER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_MULTIPLEX_BUFFER_UTILISATION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_MUX_CODE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_MVC_EXTENSION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_MVC_OPERATION_POINT);
    PRINT_CONSTANT((gint) GST_MTS_DESC_PRIVATE_DATA_INDICATOR);
    PRINT_CONSTANT((gint) GST_MTS_DESC_REGISTRATION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_RESERVED_00);
    PRINT_CONSTANT((gint) GST_MTS_DESC_RESERVED_01);
    PRINT_CONSTANT((gint) GST_MTS_DESC_SCTE_AC3);
    PRINT_CONSTANT((gint) GST_MTS_DESC_SCTE_COMPONENT_NAME);
    PRINT_CONSTANT((gint) GST_MTS_DESC_SCTE_EXTENDED_VIDEO);
    PRINT_CONSTANT((gint) GST_MTS_DESC_SCTE_FRAME_RATE);
    PRINT_CONSTANT((gint) GST_MTS_DESC_SCTE_FREQUENCY_SPEC);
    PRINT_CONSTANT((gint) GST_MTS_DESC_SCTE_MODULATION_PARAMS);
    PRINT_CONSTANT((gint) GST_MTS_DESC_SCTE_STUFFING);
    PRINT_CONSTANT((gint) GST_MTS_DESC_SCTE_TRANSPORT_STREAM_ID);
    PRINT_CONSTANT((gint) GST_MTS_DESC_SL);
    PRINT_CONSTANT((gint) GST_MTS_DESC_SMOOTHING_BUFFER);
    PRINT_CONSTANT((gint) GST_MTS_DESC_STD);
    PRINT_CONSTANT((gint) GST_MTS_DESC_STEREOSCOPIC_PROGRAM_INFO);
    PRINT_CONSTANT((gint) GST_MTS_DESC_STEREOSCOPIC_VIDEO_INFO);
    PRINT_CONSTANT((gint) GST_MTS_DESC_SVC_EXTENSION);
    PRINT_CONSTANT((gint) GST_MTS_DESC_SYSTEM_CLOCK);
    PRINT_CONSTANT((gint) GST_MTS_DESC_TARGET_BACKGROUND_GRID);
    PRINT_CONSTANT((gint) GST_MTS_DESC_VIDEO_STREAM);
    PRINT_CONSTANT((gint) GST_MTS_DESC_VIDEO_WINDOW);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_0);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_AC_3);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_AC_4);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_BSSD);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_CUEI);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_DTS1);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_DTS2);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_DTS3);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_EAC3);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_ETV1);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_GA94);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_HDMV);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_KLVA);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_OPUS);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_OTHER_HEVC);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_TSHV);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_VC_1);
    PRINT_CONSTANT((guint) GST_MTS_REGISTRATION_drac);
    PRINT_CONSTANT((gint) GST_MTS_SCTE_DESC_AUDIO);
    PRINT_CONSTANT((gint) GST_MTS_SCTE_DESC_AVAIL);
    PRINT_CONSTANT((gint) GST_MTS_SCTE_DESC_DTMF);
    PRINT_CONSTANT((gint) GST_MTS_SCTE_DESC_SEGMENTATION);
    PRINT_CONSTANT((gint) GST_MTS_SCTE_DESC_TIME);
    PRINT_CONSTANT((gint) GST_MTS_SCTE_SPLICE_COMMAND_BANDWIDTH);
    PRINT_CONSTANT((gint) GST_MTS_SCTE_SPLICE_COMMAND_INSERT);
    PRINT_CONSTANT((gint) GST_MTS_SCTE_SPLICE_COMMAND_NULL);
    PRINT_CONSTANT((gint) GST_MTS_SCTE_SPLICE_COMMAND_PRIVATE);
    PRINT_CONSTANT((gint) GST_MTS_SCTE_SPLICE_COMMAND_SCHEDULE);
    PRINT_CONSTANT((gint) GST_MTS_SCTE_SPLICE_COMMAND_TIME);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_14496_OBJET_DESCRIPTOR);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_14496_SCENE_DESCRIPTION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_14496_SECTION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_23001_10_SECTION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_23001_11_SECTION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_APPLICATION_INFORMATION_TABLE);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_AGGREGATE_DATA_EVENT);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_AGGREGATE_EVENT_INFORMATION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_AGGREGATE_EXTENDED_TEXT);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_CABLE_VIRTUAL_CHANNEL);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_CHANNEL_OR_EVENT_EXTENDED_TEXT);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_DATA_EVENT);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_DATA_SERVICE);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_DIRECTED_CHANNEL_CHANGE);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_DIRECTED_CHANNEL_CHANGE_SECTION_CODE);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_EVENT_INFORMATION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_LONG_TERM_SERVICE);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_MASTER_GUIDE);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_NETWORK_RESOURCE);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_PROGRAM_IDENTIFIER);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_RATING_REGION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_SATELLITE_VIRTUAL_CHANNEL);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_SYSTEM_TIME);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_ATSC_TERRESTRIAL_VIRTUAL_CHANNEL);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_BOUQUET_ASSOCIATION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_CA_MESSAGE_ECM_0);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_CA_MESSAGE_ECM_1);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_CA_MESSAGE_SYSTEM_PRIVATE_1);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_CA_MESSAGE_SYSTEM_PRIVATE_N);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_CMT);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_CONDITIONAL_ACCESS);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_CONTAINER);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_CONTENT_IDENTIFIER);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_DISCONTINUITY_INFORMATION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_DOWNLOADABLE_FONT_INFO);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_DSM_CC_ADDRESSABLE_SECTIONS);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_DSM_CC_DOWNLOAD_DATA_MESSAGES);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_DSM_CC_MULTIPROTO_ENCAPSULATED_DATA);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_DSM_CC_PRIVATE_DATA);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_DSM_CC_STREAM_DESCRIPTORS);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_DSM_CC_U_N_MESSAGES);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_EVENT_INFORMATION_ACTUAL_TS_PRESENT);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_EVENT_INFORMATION_ACTUAL_TS_SCHEDULE_1);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_EVENT_INFORMATION_ACTUAL_TS_SCHEDULE_N);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_EVENT_INFORMATION_OTHER_TS_PRESENT);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_EVENT_INFORMATION_OTHER_TS_SCHEDULE_1);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_EVENT_INFORMATION_OTHER_TS_SCHEDULE_N);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_FCT);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_IPMP_CONTROL_INFORMATION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_LL_FEC_PARITY_DATA_TABLE);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_METADATA);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_MPE_FEC);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_MPE_IFEC);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_NETWORK_INFORMATION_ACTUAL_NETWORK);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_NETWORK_INFORMATION_OTHER_NETWORK);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_PCR_PACKET_PAYLOAD);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_PROGRAM_ASSOCIATION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_PROTECTION_MESSAGE);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_RELATED_CONTENT);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_RESOLUTION_NOTIFICATION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_RUNNING_STATUS);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_SCT);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_SCTE_DDB);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_SCTE_DII);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_SCTE_EAS);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_SCTE_EBIF);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_SCTE_EISS);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_SCTE_RESERVED);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_SCTE_SPLICE);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_SELECTION_INFORMATION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_SERVICE_DESCRIPTION_ACTUAL_TS);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_SERVICE_DESCRIPTION_OTHER_TS);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_SPT);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_STUFFING);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_TBTP);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_TCT);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_TIM);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_TIME_DATE);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_TIME_OFFSET);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_TRANSMISSION_MODE_SUPPORT_PAYLOAD);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_TS_DESCRIPTION);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_TS_PROGRAM_MAP);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_UNSET);
    PRINT_CONSTANT((gint) GST_MTS_TABLE_ID_UPDATE_NOTIFICATION);
    PRINT_CONSTANT((gint) HEARING_IMPAIRED_PAGE);
    PRINT_CONSTANT((gint) INITIAL_PAGE);
    PRINT_CONSTANT((gint) PROGRAMME_SCHEDULE_PAGE);
    PRINT_CONSTANT((gint) SUBTITLE_PAGE);
    return 0;
}

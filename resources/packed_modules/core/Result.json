{
  "header": {
    "name": "Result",
    "exposing": {
      "Just": [
        {
          "Adt": [
            "Result",
            "All"
          ]
        },
        {
          "Definition": "withDefault"
        },
        {
          "Definition": "map"
        },
        {
          "Definition": "map2"
        },
        {
          "Definition": "map3"
        },
        {
          "Definition": "map4"
        },
        {
          "Definition": "map5"
        },
        {
          "Definition": "andThen"
        },
        {
          "Definition": "toMaybe"
        },
        {
          "Definition": "fromMaybe"
        },
        {
          "Definition": "mapError"
        }
      ]
    }
  },
  "imports": [
    {
      "path": [
        "Basics"
      ],
      "alias": null,
      "exposing": {
        "Just": [
          {
            "Adt": [
              "Bool",
              "All"
            ]
          }
        ]
      }
    },
    {
      "path": [
        "Maybe"
      ],
      "alias": null,
      "exposing": {
        "Just": [
          {
            "Adt": [
              "Maybe",
              "All"
            ]
          }
        ]
      }
    }
  ],
  "statements": [
    {
      "Adt": [
        "Result",
        [
          "error",
          "value"
        ],
        [
          [
            [
              649,
              657
            ],
            "Ok",
            [
              {
                "Var": "value"
              }
            ]
          ],
          [
            [
              664,
              673
            ],
            "Err",
            [
              {
                "Var": "error"
              }
            ]
          ]
        ]
      ]
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "a"
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Result",
                    [
                      {
                        "Var": "x"
                      },
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Var": "a"
                }
              ]
            }
          ]
        },
        "name": "withDefault",
        "patterns": [
          {
            "Var": [
              [
                966,
                969
              ],
              "def"
            ]
          },
          {
            "Var": [
              [
                970,
                976
              ],
              "result"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              981,
              1043
            ],
            {
              "Ref": [
                [
                  986,
                  992
                ],
                "result"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      1000,
                      1004
                    ],
                    "Ok",
                    [
                      {
                        "Var": [
                          [
                            1003,
                            1004
                          ],
                          "a"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Ref": [
                    [
                      1016,
                      1017
                    ],
                    "a"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      1023,
                      1028
                    ],
                    "Err",
                    [
                      {
                        "Wildcard": [
                          1027,
                          1028
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Ref": [
                    [
                      1040,
                      1043
                    ],
                    "def"
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Var": "value"
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Result",
                    [
                      {
                        "Var": "x"
                      },
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Result",
                    [
                      {
                        "Var": "x"
                      },
                      {
                        "Var": "value"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "map",
        "patterns": [
          {
            "Var": [
              [
                1347,
                1351
              ],
              "func"
            ]
          },
          {
            "Var": [
              [
                1352,
                1354
              ],
              "ra"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              1359,
              1425
            ],
            {
              "Ref": [
                [
                  1364,
                  1366
                ],
                "ra"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      1374,
                      1378
                    ],
                    "Ok",
                    [
                      {
                        "Var": [
                          [
                            1377,
                            1378
                          ],
                          "a"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      1388,
                      1399
                    ],
                    {
                      "Ref": [
                        [
                          1388,
                          1391
                        ],
                        "Ok"
                      ]
                    },
                    {
                      "Application": [
                        [
                          1392,
                          1398
                        ],
                        {
                          "Ref": [
                            [
                              1392,
                              1396
                            ],
                            "func"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              1397,
                              1398
                            ],
                            "a"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      1405,
                      1410
                    ],
                    "Err",
                    [
                      {
                        "Var": [
                          [
                            1409,
                            1410
                          ],
                          "e"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      1420,
                      1425
                    ],
                    {
                      "Ref": [
                        [
                          1420,
                          1424
                        ],
                        "Err"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          1424,
                          1425
                        ],
                        "e"
                      ]
                    }
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Var": "value"
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Result",
                    [
                      {
                        "Var": "x"
                      },
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Result",
                        [
                          {
                            "Var": "x"
                          },
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "Result",
                        [
                          {
                            "Var": "x"
                          },
                          {
                            "Var": "value"
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map2",
        "patterns": [
          {
            "Var": [
              [
                1884,
                1888
              ],
              "func"
            ]
          },
          {
            "Var": [
              [
                1889,
                1891
              ],
              "ra"
            ]
          },
          {
            "Var": [
              [
                1892,
                1894
              ],
              "rb"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              1899,
              2038
            ],
            {
              "Ref": [
                [
                  1904,
                  1906
                ],
                "ra"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      1914,
                      1919
                    ],
                    "Err",
                    [
                      {
                        "Var": [
                          [
                            1918,
                            1919
                          ],
                          "x"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      1929,
                      1934
                    ],
                    {
                      "Ref": [
                        [
                          1929,
                          1933
                        ],
                        "Err"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          1933,
                          1934
                        ],
                        "x"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      1940,
                      1944
                    ],
                    "Ok",
                    [
                      {
                        "Var": [
                          [
                            1943,
                            1944
                          ],
                          "a"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      1954,
                      2038
                    ],
                    {
                      "Ref": [
                        [
                          1959,
                          1961
                        ],
                        "rb"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            [
                              1973,
                              1978
                            ],
                            "Err",
                            [
                              {
                                "Var": [
                                  [
                                    1977,
                                    1978
                                  ],
                                  "x"
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Application": [
                            [
                              1992,
                              1997
                            ],
                            {
                              "Ref": [
                                [
                                  1992,
                                  1996
                                ],
                                "Err"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  1996,
                                  1997
                                ],
                                "x"
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            [
                              2007,
                              2011
                            ],
                            "Ok",
                            [
                              {
                                "Var": [
                                  [
                                    2010,
                                    2011
                                  ],
                                  "b"
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Application": [
                            [
                              2025,
                              2038
                            ],
                            {
                              "Ref": [
                                [
                                  2025,
                                  2028
                                ],
                                "Ok"
                              ]
                            },
                            {
                              "Application": [
                                [
                                  2029,
                                  2037
                                ],
                                {
                                  "Application": [
                                    [
                                      2029,
                                      2037
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          2029,
                                          2033
                                        ],
                                        "func"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          2034,
                                          2035
                                        ],
                                        "a"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      2036,
                                      2037
                                    ],
                                    "b"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Var": "value"
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Result",
                    [
                      {
                        "Var": "x"
                      },
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Result",
                        [
                          {
                            "Var": "x"
                          },
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "Result",
                            [
                              {
                                "Var": "x"
                              },
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Tag": [
                            "Result",
                            [
                              {
                                "Var": "x"
                              },
                              {
                                "Var": "value"
                              }
                            ]
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map3",
        "patterns": [
          {
            "Var": [
              [
                2142,
                2146
              ],
              "func"
            ]
          },
          {
            "Var": [
              [
                2147,
                2149
              ],
              "ra"
            ]
          },
          {
            "Var": [
              [
                2150,
                2152
              ],
              "rb"
            ]
          },
          {
            "Var": [
              [
                2153,
                2155
              ],
              "rc"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              2160,
              2388
            ],
            {
              "Ref": [
                [
                  2165,
                  2167
                ],
                "ra"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      2175,
                      2180
                    ],
                    "Err",
                    [
                      {
                        "Var": [
                          [
                            2179,
                            2180
                          ],
                          "x"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      2190,
                      2195
                    ],
                    {
                      "Ref": [
                        [
                          2190,
                          2194
                        ],
                        "Err"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          2194,
                          2195
                        ],
                        "x"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      2201,
                      2205
                    ],
                    "Ok",
                    [
                      {
                        "Var": [
                          [
                            2204,
                            2205
                          ],
                          "a"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      2215,
                      2388
                    ],
                    {
                      "Ref": [
                        [
                          2220,
                          2222
                        ],
                        "rb"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            [
                              2234,
                              2239
                            ],
                            "Err",
                            [
                              {
                                "Var": [
                                  [
                                    2238,
                                    2239
                                  ],
                                  "x"
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Application": [
                            [
                              2253,
                              2258
                            ],
                            {
                              "Ref": [
                                [
                                  2253,
                                  2257
                                ],
                                "Err"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  2257,
                                  2258
                                ],
                                "x"
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            [
                              2268,
                              2272
                            ],
                            "Ok",
                            [
                              {
                                "Var": [
                                  [
                                    2271,
                                    2272
                                  ],
                                  "b"
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Case": [
                            [
                              2286,
                              2388
                            ],
                            {
                              "Ref": [
                                [
                                  2291,
                                  2293
                                ],
                                "rc"
                              ]
                            },
                            [
                              [
                                {
                                  "Adt": [
                                    [
                                      2309,
                                      2314
                                    ],
                                    "Err",
                                    [
                                      {
                                        "Var": [
                                          [
                                            2313,
                                            2314
                                          ],
                                          "x"
                                        ]
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      2332,
                                      2337
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          2332,
                                          2336
                                        ],
                                        "Err"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          2336,
                                          2337
                                        ],
                                        "x"
                                      ]
                                    }
                                  ]
                                }
                              ],
                              [
                                {
                                  "Adt": [
                                    [
                                      2351,
                                      2355
                                    ],
                                    "Ok",
                                    [
                                      {
                                        "Var": [
                                          [
                                            2354,
                                            2355
                                          ],
                                          "c"
                                        ]
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      2373,
                                      2388
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          2373,
                                          2376
                                        ],
                                        "Ok"
                                      ]
                                    },
                                    {
                                      "Application": [
                                        [
                                          2377,
                                          2387
                                        ],
                                        {
                                          "Application": [
                                            [
                                              2377,
                                              2387
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  2377,
                                                  2387
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      2377,
                                                      2381
                                                    ],
                                                    "func"
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      2382,
                                                      2383
                                                    ],
                                                    "a"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  2384,
                                                  2385
                                                ],
                                                "b"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              2386,
                                              2387
                                            ],
                                            "c"
                                          ]
                                        }
                                      ]
                                    }
                                  ]
                                }
                              ]
                            ]
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Fun": [
                            {
                              "Var": "d"
                            },
                            {
                              "Var": "value"
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Result",
                    [
                      {
                        "Var": "x"
                      },
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Result",
                        [
                          {
                            "Var": "x"
                          },
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "Result",
                            [
                              {
                                "Var": "x"
                              },
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Fun": [
                            {
                              "Tag": [
                                "Result",
                                [
                                  {
                                    "Var": "x"
                                  },
                                  {
                                    "Var": "d"
                                  }
                                ]
                              ]
                            },
                            {
                              "Tag": [
                                "Result",
                                [
                                  {
                                    "Var": "x"
                                  },
                                  {
                                    "Var": "value"
                                  }
                                ]
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map4",
        "patterns": [
          {
            "Var": [
              [
                2511,
                2515
              ],
              "func"
            ]
          },
          {
            "Var": [
              [
                2516,
                2518
              ],
              "ra"
            ]
          },
          {
            "Var": [
              [
                2519,
                2521
              ],
              "rb"
            ]
          },
          {
            "Var": [
              [
                2522,
                2524
              ],
              "rc"
            ]
          },
          {
            "Var": [
              [
                2525,
                2527
              ],
              "rd"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              2532,
              2865
            ],
            {
              "Ref": [
                [
                  2537,
                  2539
                ],
                "ra"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      2547,
                      2552
                    ],
                    "Err",
                    [
                      {
                        "Var": [
                          [
                            2551,
                            2552
                          ],
                          "x"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      2562,
                      2567
                    ],
                    {
                      "Ref": [
                        [
                          2562,
                          2566
                        ],
                        "Err"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          2566,
                          2567
                        ],
                        "x"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      2573,
                      2577
                    ],
                    "Ok",
                    [
                      {
                        "Var": [
                          [
                            2576,
                            2577
                          ],
                          "a"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      2587,
                      2865
                    ],
                    {
                      "Ref": [
                        [
                          2592,
                          2594
                        ],
                        "rb"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            [
                              2606,
                              2611
                            ],
                            "Err",
                            [
                              {
                                "Var": [
                                  [
                                    2610,
                                    2611
                                  ],
                                  "x"
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Application": [
                            [
                              2625,
                              2630
                            ],
                            {
                              "Ref": [
                                [
                                  2625,
                                  2629
                                ],
                                "Err"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  2629,
                                  2630
                                ],
                                "x"
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            [
                              2640,
                              2644
                            ],
                            "Ok",
                            [
                              {
                                "Var": [
                                  [
                                    2643,
                                    2644
                                  ],
                                  "b"
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Case": [
                            [
                              2658,
                              2865
                            ],
                            {
                              "Ref": [
                                [
                                  2663,
                                  2665
                                ],
                                "rc"
                              ]
                            },
                            [
                              [
                                {
                                  "Adt": [
                                    [
                                      2681,
                                      2686
                                    ],
                                    "Err",
                                    [
                                      {
                                        "Var": [
                                          [
                                            2685,
                                            2686
                                          ],
                                          "x"
                                        ]
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      2704,
                                      2709
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          2704,
                                          2708
                                        ],
                                        "Err"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          2708,
                                          2709
                                        ],
                                        "x"
                                      ]
                                    }
                                  ]
                                }
                              ],
                              [
                                {
                                  "Adt": [
                                    [
                                      2723,
                                      2727
                                    ],
                                    "Ok",
                                    [
                                      {
                                        "Var": [
                                          [
                                            2726,
                                            2727
                                          ],
                                          "c"
                                        ]
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Case": [
                                    [
                                      2745,
                                      2865
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          2750,
                                          2752
                                        ],
                                        "rd"
                                      ]
                                    },
                                    [
                                      [
                                        {
                                          "Adt": [
                                            [
                                              2772,
                                              2777
                                            ],
                                            "Err",
                                            [
                                              {
                                                "Var": [
                                                  [
                                                    2776,
                                                    2777
                                                  ],
                                                  "x"
                                                ]
                                              }
                                            ]
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              2799,
                                              2804
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  2799,
                                                  2803
                                                ],
                                                "Err"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  2803,
                                                  2804
                                                ],
                                                "x"
                                              ]
                                            }
                                          ]
                                        }
                                      ],
                                      [
                                        {
                                          "Adt": [
                                            [
                                              2822,
                                              2826
                                            ],
                                            "Ok",
                                            [
                                              {
                                                "Var": [
                                                  [
                                                    2825,
                                                    2826
                                                  ],
                                                  "d"
                                                ]
                                              }
                                            ]
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              2848,
                                              2865
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  2848,
                                                  2851
                                                ],
                                                "Ok"
                                              ]
                                            },
                                            {
                                              "Application": [
                                                [
                                                  2852,
                                                  2864
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      2852,
                                                      2864
                                                    ],
                                                    {
                                                      "Application": [
                                                        [
                                                          2852,
                                                          2864
                                                        ],
                                                        {
                                                          "Application": [
                                                            [
                                                              2852,
                                                              2864
                                                            ],
                                                            {
                                                              "Ref": [
                                                                [
                                                                  2852,
                                                                  2856
                                                                ],
                                                                "func"
                                                              ]
                                                            },
                                                            {
                                                              "Ref": [
                                                                [
                                                                  2857,
                                                                  2858
                                                                ],
                                                                "a"
                                                              ]
                                                            }
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              2859,
                                                              2860
                                                            ],
                                                            "b"
                                                          ]
                                                        }
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          2861,
                                                          2862
                                                        ],
                                                        "c"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      2863,
                                                      2864
                                                    ],
                                                    "d"
                                                  ]
                                                }
                                              ]
                                            }
                                          ]
                                        }
                                      ]
                                    ]
                                  ]
                                }
                              ]
                            ]
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Fun": [
                            {
                              "Var": "d"
                            },
                            {
                              "Fun": [
                                {
                                  "Var": "e"
                                },
                                {
                                  "Var": "value"
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Result",
                    [
                      {
                        "Var": "x"
                      },
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Result",
                        [
                          {
                            "Var": "x"
                          },
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "Result",
                            [
                              {
                                "Var": "x"
                              },
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Fun": [
                            {
                              "Tag": [
                                "Result",
                                [
                                  {
                                    "Var": "x"
                                  },
                                  {
                                    "Var": "d"
                                  }
                                ]
                              ]
                            },
                            {
                              "Fun": [
                                {
                                  "Tag": [
                                    "Result",
                                    [
                                      {
                                        "Var": "x"
                                      },
                                      {
                                        "Var": "e"
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Tag": [
                                    "Result",
                                    [
                                      {
                                        "Var": "x"
                                      },
                                      {
                                        "Var": "value"
                                      }
                                    ]
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map5",
        "patterns": [
          {
            "Var": [
              [
                3007,
                3011
              ],
              "func"
            ]
          },
          {
            "Var": [
              [
                3012,
                3014
              ],
              "ra"
            ]
          },
          {
            "Var": [
              [
                3015,
                3017
              ],
              "rb"
            ]
          },
          {
            "Var": [
              [
                3018,
                3020
              ],
              "rc"
            ]
          },
          {
            "Var": [
              [
                3021,
                3023
              ],
              "rd"
            ]
          },
          {
            "Var": [
              [
                3024,
                3026
              ],
              "re"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              3031,
              3485
            ],
            {
              "Ref": [
                [
                  3036,
                  3038
                ],
                "ra"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      3046,
                      3051
                    ],
                    "Err",
                    [
                      {
                        "Var": [
                          [
                            3050,
                            3051
                          ],
                          "x"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      3061,
                      3066
                    ],
                    {
                      "Ref": [
                        [
                          3061,
                          3065
                        ],
                        "Err"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          3065,
                          3066
                        ],
                        "x"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      3072,
                      3076
                    ],
                    "Ok",
                    [
                      {
                        "Var": [
                          [
                            3075,
                            3076
                          ],
                          "a"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      3086,
                      3485
                    ],
                    {
                      "Ref": [
                        [
                          3091,
                          3093
                        ],
                        "rb"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            [
                              3105,
                              3110
                            ],
                            "Err",
                            [
                              {
                                "Var": [
                                  [
                                    3109,
                                    3110
                                  ],
                                  "x"
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Application": [
                            [
                              3124,
                              3129
                            ],
                            {
                              "Ref": [
                                [
                                  3124,
                                  3128
                                ],
                                "Err"
                              ]
                            },
                            {
                              "Ref": [
                                [
                                  3128,
                                  3129
                                ],
                                "x"
                              ]
                            }
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            [
                              3139,
                              3143
                            ],
                            "Ok",
                            [
                              {
                                "Var": [
                                  [
                                    3142,
                                    3143
                                  ],
                                  "b"
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Case": [
                            [
                              3157,
                              3485
                            ],
                            {
                              "Ref": [
                                [
                                  3162,
                                  3164
                                ],
                                "rc"
                              ]
                            },
                            [
                              [
                                {
                                  "Adt": [
                                    [
                                      3180,
                                      3185
                                    ],
                                    "Err",
                                    [
                                      {
                                        "Var": [
                                          [
                                            3184,
                                            3185
                                          ],
                                          "x"
                                        ]
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      3203,
                                      3208
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          3203,
                                          3207
                                        ],
                                        "Err"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          3207,
                                          3208
                                        ],
                                        "x"
                                      ]
                                    }
                                  ]
                                }
                              ],
                              [
                                {
                                  "Adt": [
                                    [
                                      3222,
                                      3226
                                    ],
                                    "Ok",
                                    [
                                      {
                                        "Var": [
                                          [
                                            3225,
                                            3226
                                          ],
                                          "c"
                                        ]
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Case": [
                                    [
                                      3244,
                                      3485
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          3249,
                                          3251
                                        ],
                                        "rd"
                                      ]
                                    },
                                    [
                                      [
                                        {
                                          "Adt": [
                                            [
                                              3271,
                                              3276
                                            ],
                                            "Err",
                                            [
                                              {
                                                "Var": [
                                                  [
                                                    3275,
                                                    3276
                                                  ],
                                                  "x"
                                                ]
                                              }
                                            ]
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              3298,
                                              3303
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  3298,
                                                  3302
                                                ],
                                                "Err"
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  3302,
                                                  3303
                                                ],
                                                "x"
                                              ]
                                            }
                                          ]
                                        }
                                      ],
                                      [
                                        {
                                          "Adt": [
                                            [
                                              3321,
                                              3325
                                            ],
                                            "Ok",
                                            [
                                              {
                                                "Var": [
                                                  [
                                                    3324,
                                                    3325
                                                  ],
                                                  "d"
                                                ]
                                              }
                                            ]
                                          ]
                                        },
                                        {
                                          "Case": [
                                            [
                                              3347,
                                              3485
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  3352,
                                                  3354
                                                ],
                                                "re"
                                              ]
                                            },
                                            [
                                              [
                                                {
                                                  "Adt": [
                                                    [
                                                      3378,
                                                      3383
                                                    ],
                                                    "Err",
                                                    [
                                                      {
                                                        "Var": [
                                                          [
                                                            3382,
                                                            3383
                                                          ],
                                                          "x"
                                                        ]
                                                      }
                                                    ]
                                                  ]
                                                },
                                                {
                                                  "Application": [
                                                    [
                                                      3409,
                                                      3414
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          3409,
                                                          3413
                                                        ],
                                                        "Err"
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          3413,
                                                          3414
                                                        ],
                                                        "x"
                                                      ]
                                                    }
                                                  ]
                                                }
                                              ],
                                              [
                                                {
                                                  "Adt": [
                                                    [
                                                      3436,
                                                      3440
                                                    ],
                                                    "Ok",
                                                    [
                                                      {
                                                        "Var": [
                                                          [
                                                            3439,
                                                            3440
                                                          ],
                                                          "e"
                                                        ]
                                                      }
                                                    ]
                                                  ]
                                                },
                                                {
                                                  "Application": [
                                                    [
                                                      3466,
                                                      3485
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          3466,
                                                          3469
                                                        ],
                                                        "Ok"
                                                      ]
                                                    },
                                                    {
                                                      "Application": [
                                                        [
                                                          3470,
                                                          3484
                                                        ],
                                                        {
                                                          "Application": [
                                                            [
                                                              3470,
                                                              3484
                                                            ],
                                                            {
                                                              "Application": [
                                                                [
                                                                  3470,
                                                                  3484
                                                                ],
                                                                {
                                                                  "Application": [
                                                                    [
                                                                      3470,
                                                                      3484
                                                                    ],
                                                                    {
                                                                      "Application": [
                                                                        [
                                                                          3470,
                                                                          3484
                                                                        ],
                                                                        {
                                                                          "Ref": [
                                                                            [
                                                                              3470,
                                                                              3474
                                                                            ],
                                                                            "func"
                                                                          ]
                                                                        },
                                                                        {
                                                                          "Ref": [
                                                                            [
                                                                              3475,
                                                                              3476
                                                                            ],
                                                                            "a"
                                                                          ]
                                                                        }
                                                                      ]
                                                                    },
                                                                    {
                                                                      "Ref": [
                                                                        [
                                                                          3477,
                                                                          3478
                                                                        ],
                                                                        "b"
                                                                      ]
                                                                    }
                                                                  ]
                                                                },
                                                                {
                                                                  "Ref": [
                                                                    [
                                                                      3479,
                                                                      3480
                                                                    ],
                                                                    "c"
                                                                  ]
                                                                }
                                                              ]
                                                            },
                                                            {
                                                              "Ref": [
                                                                [
                                                                  3481,
                                                                  3482
                                                                ],
                                                                "d"
                                                              ]
                                                            }
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              3483,
                                                              3484
                                                            ],
                                                            "e"
                                                          ]
                                                        }
                                                      ]
                                                    }
                                                  ]
                                                }
                                              ]
                                            ]
                                          ]
                                        }
                                      ]
                                    ]
                                  ]
                                }
                              ]
                            ]
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Tag": [
                    "Result",
                    [
                      {
                        "Var": "x"
                      },
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Result",
                    [
                      {
                        "Var": "x"
                      },
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Result",
                    [
                      {
                        "Var": "x"
                      },
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "andThen",
        "patterns": [
          {
            "Var": [
              [
                4757,
                4765
              ],
              "callback"
            ]
          },
          {
            "Var": [
              [
                4766,
                4772
              ],
              "result"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              4779,
              4868
            ],
            {
              "Ref": [
                [
                  4784,
                  4790
                ],
                "result"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      4800,
                      4808
                    ],
                    "Ok",
                    [
                      {
                        "Var": [
                          [
                            4803,
                            4808
                          ],
                          "value"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      4820,
                      4834
                    ],
                    {
                      "Ref": [
                        [
                          4820,
                          4828
                        ],
                        "callback"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          4829,
                          4834
                        ],
                        "value"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      4842,
                      4849
                    ],
                    "Err",
                    [
                      {
                        "Var": [
                          [
                            4846,
                            4849
                          ],
                          "msg"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      4861,
                      4868
                    ],
                    {
                      "Ref": [
                        [
                          4861,
                          4865
                        ],
                        "Err"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          4865,
                          4868
                        ],
                        "msg"
                      ]
                    }
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "x"
                },
                {
                  "Var": "y"
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Result",
                    [
                      {
                        "Var": "x"
                      },
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Result",
                    [
                      {
                        "Var": "y"
                      },
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "mapError",
        "patterns": [
          {
            "Var": [
              [
                5312,
                5313
              ],
              "f"
            ]
          },
          {
            "Var": [
              [
                5314,
                5320
              ],
              "result"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              5327,
              5402
            ],
            {
              "Ref": [
                [
                  5332,
                  5338
                ],
                "result"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      5348,
                      5352
                    ],
                    "Ok",
                    [
                      {
                        "Var": [
                          [
                            5351,
                            5352
                          ],
                          "v"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      5364,
                      5368
                    ],
                    {
                      "Ref": [
                        [
                          5364,
                          5367
                        ],
                        "Ok"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          5367,
                          5368
                        ],
                        "v"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      5376,
                      5381
                    ],
                    "Err",
                    [
                      {
                        "Var": [
                          [
                            5380,
                            5381
                          ],
                          "e"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      5393,
                      5402
                    ],
                    {
                      "Ref": [
                        [
                          5393,
                          5397
                        ],
                        "Err"
                      ]
                    },
                    {
                      "Application": [
                        [
                          5398,
                          5401
                        ],
                        {
                          "Ref": [
                            [
                              5398,
                              5399
                            ],
                            "f"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              5400,
                              5401
                            ],
                            "e"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Result",
                [
                  {
                    "Var": "x"
                  },
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Maybe",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            }
          ]
        },
        "name": "toMaybe",
        "patterns": [
          {
            "Var": [
              [
                5740,
                5746
              ],
              "result"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              5753,
              5812
            ],
            {
              "Ref": [
                [
                  5758,
                  5764
                ],
                "result"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      5774,
                      5779
                    ],
                    "Ok",
                    [
                      {
                        "Var": [
                          [
                            5778,
                            5779
                          ],
                          "v"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      5783,
                      5789
                    ],
                    {
                      "Ref": [
                        [
                          5783,
                          5788
                        ],
                        "Just"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          5788,
                          5789
                        ],
                        "v"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      5796,
                      5801
                    ],
                    "Err",
                    [
                      {
                        "Wildcard": [
                          5800,
                          5801
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Ref": [
                    [
                      5805,
                      5812
                    ],
                    "Nothing"
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "x"
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Result",
                    [
                      {
                        "Var": "x"
                      },
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "fromMaybe",
        "patterns": [
          {
            "Var": [
              [
                6155,
                6158
              ],
              "err"
            ]
          },
          {
            "Var": [
              [
                6159,
                6164
              ],
              "maybe"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              6171,
              6231
            ],
            {
              "Ref": [
                [
                  6176,
                  6181
                ],
                "maybe"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      6191,
                      6197
                    ],
                    "Just",
                    [
                      {
                        "Var": [
                          [
                            6196,
                            6197
                          ],
                          "v"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      6202,
                      6206
                    ],
                    {
                      "Ref": [
                        [
                          6202,
                          6205
                        ],
                        "Ok"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          6205,
                          6206
                        ],
                        "v"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      6213,
                      6220
                    ],
                    "Nothing",
                    []
                  ]
                },
                {
                  "Application": [
                    [
                      6224,
                      6231
                    ],
                    {
                      "Ref": [
                        [
                          6224,
                          6228
                        ],
                        "Err"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          6228,
                          6231
                        ],
                        "err"
                      ]
                    }
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Result",
                [
                  {
                    "Var": "x"
                  },
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Bool",
                []
              ]
            }
          ]
        },
        "name": "isOk",
        "patterns": [
          {
            "Var": [
              [
                6344,
                6350
              ],
              "result"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              6355,
              6418
            ],
            {
              "Ref": [
                [
                  6360,
                  6366
                ],
                "result"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      6374,
                      6378
                    ],
                    "Ok",
                    [
                      {
                        "Wildcard": [
                          6377,
                          6378
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Ref": [
                    [
                      6388,
                      6392
                    ],
                    "True"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      6398,
                      6403
                    ],
                    "Err",
                    [
                      {
                        "Wildcard": [
                          6402,
                          6403
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Ref": [
                    [
                      6413,
                      6418
                    ],
                    "False"
                  ]
                }
              ]
            ]
          ]
        }
      }
    }
  ]
}
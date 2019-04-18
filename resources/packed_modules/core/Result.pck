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
            "Ok",
            [
              {
                "Var": "value"
              }
            ]
          ],
          [
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
            "Var": "def"
          },
          {
            "Var": "result"
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
                    "Ok",
                    [
                      {
                        "Var": "a"
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
                    "Err",
                    [
                      "Wildcard"
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
            "Var": "func"
          },
          {
            "Var": "ra"
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
                    "Ok",
                    [
                      {
                        "Var": "a"
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
                    "Err",
                    [
                      {
                        "Var": "e"
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
            "Var": "func"
          },
          {
            "Var": "ra"
          },
          {
            "Var": "rb"
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
                    "Err",
                    [
                      {
                        "Var": "x"
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
                    "Ok",
                    [
                      {
                        "Var": "a"
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
                            "Err",
                            [
                              {
                                "Var": "x"
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
                            "Ok",
                            [
                              {
                                "Var": "b"
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
            "Var": "func"
          },
          {
            "Var": "ra"
          },
          {
            "Var": "rb"
          },
          {
            "Var": "rc"
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
                    "Err",
                    [
                      {
                        "Var": "x"
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
                    "Ok",
                    [
                      {
                        "Var": "a"
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
                            "Err",
                            [
                              {
                                "Var": "x"
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
                            "Ok",
                            [
                              {
                                "Var": "b"
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
                                    "Err",
                                    [
                                      {
                                        "Var": "x"
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
                                    "Ok",
                                    [
                                      {
                                        "Var": "c"
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
            "Var": "func"
          },
          {
            "Var": "ra"
          },
          {
            "Var": "rb"
          },
          {
            "Var": "rc"
          },
          {
            "Var": "rd"
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
                    "Err",
                    [
                      {
                        "Var": "x"
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
                    "Ok",
                    [
                      {
                        "Var": "a"
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
                            "Err",
                            [
                              {
                                "Var": "x"
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
                            "Ok",
                            [
                              {
                                "Var": "b"
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
                                    "Err",
                                    [
                                      {
                                        "Var": "x"
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
                                    "Ok",
                                    [
                                      {
                                        "Var": "c"
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
                                            "Err",
                                            [
                                              {
                                                "Var": "x"
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
                                            "Ok",
                                            [
                                              {
                                                "Var": "d"
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
            "Var": "func"
          },
          {
            "Var": "ra"
          },
          {
            "Var": "rb"
          },
          {
            "Var": "rc"
          },
          {
            "Var": "rd"
          },
          {
            "Var": "re"
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
                    "Err",
                    [
                      {
                        "Var": "x"
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
                    "Ok",
                    [
                      {
                        "Var": "a"
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
                            "Err",
                            [
                              {
                                "Var": "x"
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
                            "Ok",
                            [
                              {
                                "Var": "b"
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
                                    "Err",
                                    [
                                      {
                                        "Var": "x"
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
                                    "Ok",
                                    [
                                      {
                                        "Var": "c"
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
                                            "Err",
                                            [
                                              {
                                                "Var": "x"
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
                                            "Ok",
                                            [
                                              {
                                                "Var": "d"
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
                                                    "Err",
                                                    [
                                                      {
                                                        "Var": "x"
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
                                                    "Ok",
                                                    [
                                                      {
                                                        "Var": "e"
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
            "Var": "callback"
          },
          {
            "Var": "result"
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
                    "Ok",
                    [
                      {
                        "Var": "value"
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
                    "Err",
                    [
                      {
                        "Var": "msg"
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
            "Var": "f"
          },
          {
            "Var": "result"
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
                    "Ok",
                    [
                      {
                        "Var": "v"
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
                    "Err",
                    [
                      {
                        "Var": "e"
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
            "Var": "result"
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
                    "Ok",
                    [
                      {
                        "Var": "v"
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
                    "Err",
                    [
                      "Wildcard"
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
            "Var": "err"
          },
          {
            "Var": "maybe"
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
                    "Just",
                    [
                      {
                        "Var": "v"
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
            "Var": "result"
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
                    "Ok",
                    [
                      "Wildcard"
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
                    "Err",
                    [
                      "Wildcard"
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